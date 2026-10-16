// Due-date dialog scope check: every save may change only the open task's due_at.
//
// node due-scope.mjs --base URL --jar NETSCAPE_COOKIES --board BOARD_ID --out RESULT.json
//
// Snapshots every task's due_at via the API before and after each dialog
// save (add, change, other column, clear) and diffs them. Exit 0 only when
// every step changed exactly the open task, to the expected value, with
// zero console/page errors.
import { readFileSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';

process.env.PLAYWRIGHT_BROWSERS_PATH ??= '0';
const PW = '/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/r2/pw/node_modules/playwright/index.js';
const { chromium } = (await import(PW)).default;
const playwrightVersion = createRequire(PW)('playwright/package.json').version;

const args = {};
for (let i = 2; i < process.argv.length; i += 2) args[process.argv[i].replace(/^--/, '')] = process.argv[i + 1];
for (const k of ['base', 'jar', 'board', 'out']) if (!args[k]) throw new Error(`--${k} is required`);

function readCookieJar(file) {
  const cookies = [];
  for (const raw of readFileSync(file, 'utf8').split('\n')) {
    let line = raw.trim();
    let httpOnly = false;
    if (line.startsWith('#HttpOnly_')) { httpOnly = true; line = line.slice(10); }
    else if (line === '' || line.startsWith('#')) continue;
    const [domain, , p, secure, expires, name, value = ''] = line.split('\t');
    if (!name) continue;
    const e = Number(expires);
    cookies.push({ name, value, domain: domain.replace(/^\./, ''), path: p, httpOnly,
      secure: secure === 'TRUE', expires: e > 0 ? e : -1 });
  }
  return cookies;
}

const log = { base: args.base, board: args.board, playwrightVersion, started: new Date().toISOString(),
  consoleErrors: [], pageErrors: [], httpErrors: [], steps: [] };
const browser = await chromium.launch();
const ctx = await browser.newContext({ viewport: { width: 1905, height: 1000 }, deviceScaleFactor: 1, locale: 'en-US' });
await ctx.addCookies(readCookieJar(args.jar));
const page = await ctx.newPage();
page.on('console', (m) => { if (m.type() === 'error') log.consoleErrors.push({ text: m.text(), loc: m.location() }); });
page.on('pageerror', (e) => log.pageErrors.push(String(e)));
page.on('response', (r) => { if (r.status() >= 400) log.httpErrors.push({ url: r.url(), status: r.status() }); });

const getJson = async (url) => {
  const r = await ctx.request.get(args.base + url, { headers: { Accept: 'application/json' } });
  if (!r.ok()) throw new Error(`GET ${url} -> ${r.status()}`);
  return r.json();
};

// Every task on the instance (all boards): id -> { name, due_at }.
async function snapshot() {
  const list = await getJson('/api/tasks');
  const out = {};
  for (const t of list) {
    const d = await getJson(`/api/tasks/${encodeURIComponent(t.id)}`);
    out[t.id] = { name: d.name, due_at: d.due_at ?? null };
  }
  return out;
}

function diff(before, after) {
  const changed = [];
  for (const id of new Set([...Object.keys(before), ...Object.keys(after)])) {
    const a = before[id]?.due_at ?? null;
    const b = after[id]?.due_at ?? null;
    if (a !== b) changed.push({ id, name: (after[id] ?? before[id]).name, before: a, after: b });
  }
  return changed;
}

// The open task: the first card in the board column holding the most cards.
await page.goto(`${args.base}/b/${args.board}`, { waitUntil: 'networkidle' });
await page.waitForTimeout(500);
const layout = await page.evaluate(() => {
  const cols = {};
  for (const card of document.querySelectorAll('.task-list[data-column-id] .task-card[data-task-id]')) {
    const col = card.closest('.task-list').getAttribute('data-column-id');
    (cols[col] ??= []).push(card.dataset.taskId);
  }
  return cols;
});
const [ownCol, ownIds] = Object.entries(layout).sort((a, b) => b[1].length - a[1].length)[0];
const otherCol = Object.entries(layout).filter(([c, ids]) => c !== ownCol && ids.length).sort((a, b) => b[1].length - a[1].length)[0]?.[0];
const taskId = ownIds[0];
log.openTask = taskId;
log.ownColumn = { id: ownCol, cards: ownIds.length };
log.otherColumn = { id: otherCol, cards: otherCol ? layout[otherCol].length : 0 };

await page.click(`.task-card[data-task-id="${taskId}"] .card-title-text`);
await page.waitForSelector('#modal-overlay .task-modal');
await page.waitForTimeout(300);

async function dialogSave(label, { date, time, column }) {
  const before = await snapshot();
  await page.click('[data-tm-menu="tm-add-menu"]');
  await page.click('#tm-add-menu [data-tm-act="add-duedate"]');
  await page.waitForSelector('#duedate-dialog:not([hidden])');
  await page.waitForTimeout(300);
  const selectedBefore = await page.$eval('#dd-column', (s) => s.value);
  // The date field is readonly: pick through the inline calendar. The dialog
  // has no clear control, so the clear step empties the field in script.
  if (date) await page.click(`#dd-cal-inline [data-date="${date}"]`);
  else await page.$eval('#dd-date', (i) => { i.value = ''; });
  await page.fill('#dd-time', time);
  const picked = await page.$eval('#dd-date', (i) => i.value);
  if (picked !== date) throw new Error(`calendar pick failed: wanted ${date}, field shows ${picked}`);
  if (column) await page.selectOption('#dd-column', column);
  const expected = date
    ? await page.evaluate(([d, t]) => new Date(`${d}T${t}:00`).toISOString(), [date, time])
    : null;
  await page.click('#dd-add');
  await page.waitForTimeout(900);
  const toast = await page.$eval('#toast', (t) => t.innerText).catch(() => null);
  const dialogHidden = await page.$eval('#duedate-dialog', (d) => d.hidden);
  const after = await snapshot();
  const changed = diff(before, after);
  const openAfter = after[taskId].due_at;
  const others = changed.filter((c) => c.id !== taskId);
  const ok = others.length === 0 && openAfter === expected;
  const step = { label, date, time, column: column ?? null, selectedBefore, expected, openTaskAfter: openAfter,
    toast, dialogHidden, changedCount: changed.length, otherTasksChanged: others.length, changed, ok };
  log.steps.push(step);
  console.log(`${ok ? 'PASS' : 'FAIL'} ${label}: open task ${openAfter} (expected ${expected}); other tasks changed: ${others.length}; toast: ${toast}`);
}

try {
  log.baseline = await snapshot();
  await dialogSave('add', { date: '2026-10-20', time: '10:00' });
  await dialogSave('change', { date: '2026-10-22', time: '14:30' });
  await dialogSave('other-column', { date: '2026-10-27', time: '09:15', column: otherCol });
  await dialogSave('clear', { date: '', time: '' });
  log.final = await snapshot();
} catch (e) {
  log.error = String(e?.stack ?? e);
  console.log('ERROR', log.error);
}
log.finished = new Date().toISOString();
log.ok = !log.error && log.steps.length === 4 && log.steps.every((s) => s.ok)
  && log.consoleErrors.length === 0 && log.pageErrors.length === 0;
writeFileSync(args.out, JSON.stringify(log, null, 2));
console.log(`== ${args.base}: ok=${log.ok} consoleErrors=${log.consoleErrors.length} pageErrors=${log.pageErrors.length} http>=400=${log.httpErrors.length}`);
await browser.close();
process.exit(log.ok ? 0 : 1);
