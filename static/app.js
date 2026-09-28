/* Pomodoro Kanban frontend glue: drag-and-drop, task modal, pomodoro timer.
 * No framework — plain JS plus SortableJS (drag-and-drop) and htmx
 * (add-task forms, rendered server-side). */
(function () {
  'use strict';

  var dragging = false;   // true while a Sortable drag is in flight (suppresses card clicks)
  var modalDirty = false; // set when the modal changed something; reloads the board on close
  var timer = null;       // active timer state, or null

  // ---------- drag and drop ----------

  function initSortable() {
    if (typeof Sortable === 'undefined') return;
    document.querySelectorAll('.task-list').forEach(function (list) {
      if (list._sortable) return;
      list._sortable = new Sortable(list, {
        group: 'tasks',
        animation: 150,
        draggable: '.task-card',
        ghostClass: 'drag-ghost',
        onStart: function () { dragging = true; },
        onEnd: function (evt) {
          window.setTimeout(function () { dragging = false; }, 80);
          var card = evt.item;
          var toList = evt.to;
          var cards = Array.prototype.slice.call(toList.querySelectorAll('.task-card'));
          var position = cards.indexOf(card);
          fetch('/api/tasks/' + encodeURIComponent(card.dataset.taskId) + '/move', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              column_id: toList.dataset.columnId,
              swimlane_id: toList.dataset.swimlaneId || null,
              position: position,
            }),
          }).then(function (res) {
            if (!res.ok) window.location.reload(); // resync the board on failure
          }).catch(function () { window.location.reload(); });
        },
      });
    });
  }
  // Note: htmx appends new cards inside the existing .task-list containers,
  // which already have Sortable attached, so no re-init is needed.

  // ---------- task modal ----------

  function modalTaskId() {
    var modal = document.querySelector('#modal-root .modal');
    return modal ? modal.dataset.taskId : null;
  }

  function openModal(taskId) {
    fetch('/api/tasks/' + encodeURIComponent(taskId) + '/modal', {
      headers: { 'Accept': 'text/html' },
    })
      .then(function (res) { return res.ok ? res.text() : Promise.reject(res.status); })
      .then(function (html) {
        document.getElementById('modal-root').innerHTML = html;
        modalDirty = false;
        wireModal();
      })
      .catch(function () { /* leave the board as-is on failure */ });
  }

  function closeModal() {
    stopTimer();
    document.removeEventListener('keydown', escHandler);
    document.getElementById('modal-root').innerHTML = '';
    if (modalDirty) window.location.reload();
  }

  function escHandler(e) {
    if (e.key === 'Escape') closeModal();
  }

  function wireModal() {
    var overlay = document.getElementById('modal-overlay');
    if (!overlay) return;
    overlay.addEventListener('click', function (e) {
      if (e.target === overlay) closeModal();
    });
    document.addEventListener('keydown', escHandler);

    // Size picker: PATCH the size, highlight the choice, mark dirty.
    overlay.querySelectorAll('.size-picker button').forEach(function (btn) {
      btn.addEventListener('click', function () {
        var id = modalTaskId();
        fetch('/api/tasks/' + encodeURIComponent(id), {
          method: 'PATCH',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ size: parseInt(btn.dataset.size, 10) }),
        }).then(function (res) {
          if (res.ok) {
            overlay.querySelectorAll('.size-picker button').forEach(function (b) {
              b.classList.remove('selected');
            });
            btn.classList.add('selected');
            modalDirty = true;
          }
        });
      });
    });

    // Manual "log time" form: POST, then swap in the refreshed entries list.
    var form = document.getElementById('log-time-form');
    form.addEventListener('submit', function (e) {
      e.preventDefault();
      var id = modalTaskId();
      fetch('/api/tasks/' + encodeURIComponent(id) + '/time', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          minutes: parseInt(form.minutes.value, 10),
          note: form.note.value,
        }),
      })
        .then(function (res) { return res.ok ? res.text() : Promise.reject(res.status); })
        .then(function (html) {
          document.getElementById('time-entries').innerHTML = html;
          form.note.value = '';
          modalDirty = true;
        })
        .catch(function () { /* keep the form open on failure */ });
    });

    initTimer();
  }

  function saveModalTask() {
    var id = modalTaskId();
    var name = document.getElementById('modal-name').value.trim();
    var description = document.getElementById('modal-description').value;
    if (!name || !id) return;
    fetch('/api/tasks/' + encodeURIComponent(id), {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name: name, description: description }),
    }).then(function (res) { if (res.ok) modalDirty = true; });
  }

  function deleteModalTask() {
    var id = modalTaskId();
    if (!id || !window.confirm('Delete this task and its time entries?')) return;
    fetch('/api/tasks/' + encodeURIComponent(id), { method: 'DELETE' })
      .then(function (res) {
        if (res.ok) { modalDirty = true; closeModal(); }
      });
  }

  // ---------- pomodoro timer ----------

  function initTimer() {
    var modal = document.querySelector('#modal-root .modal');
    if (!modal) return;
    var total = parseInt(modal.dataset.pomodoroMinutes, 10) * 60;
    if (!total || total <= 0) total = 1500;
    timer = { total: total, remaining: total, interval: null, taskId: modal.dataset.taskId };
    renderTimer();
    document.getElementById('timer-start').addEventListener('click', startTimer);
    document.getElementById('timer-pause').addEventListener('click', pauseTimer);
    document.getElementById('timer-reset').addEventListener('click', resetTimer);
  }

  function renderTimer() {
    var display = document.getElementById('timer-display');
    if (!display || !timer) return;
    var m = Math.floor(timer.remaining / 60);
    var s = timer.remaining % 60;
    display.textContent =
      String(m).padStart(2, '0') + ':' + String(s).padStart(2, '0');
    var running = timer.interval !== null;
    document.getElementById('timer-start').disabled = running;
    document.getElementById('timer-pause').disabled = !running;
  }

  function startTimer() {
    if (!timer || timer.interval !== null) return;
    timer.interval = window.setInterval(function () {
      timer.remaining -= 1;
      if (timer.remaining <= 0) { finishTimer(); return; }
      renderTimer();
    }, 1000);
    renderTimer();
  }

  function pauseTimer() {
    if (!timer || timer.interval === null) return;
    window.clearInterval(timer.interval);
    timer.interval = null;
    renderTimer();
  }

  function resetTimer() {
    if (!timer) return;
    pauseTimer();
    timer.remaining = timer.total;
    renderTimer();
  }

  function stopTimer() {
    if (timer && timer.interval !== null) window.clearInterval(timer.interval);
    timer = null;
  }

  // When the countdown hits zero, auto-log one pomodoro on the task.
  function finishTimer() {
    var taskId = timer.taskId;
    var minutes = Math.round(timer.total / 60);
    window.clearInterval(timer.interval);
    timer.interval = null;
    timer.remaining = timer.total;
    renderTimer();
    var display = document.getElementById('timer-display');
    if (display) display.textContent = 'Done!';
    fetch('/api/tasks/' + encodeURIComponent(taskId) + '/time', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ minutes: minutes, note: 'Pomodoro' }),
    })
      .then(function (res) { return res.ok ? res.text() : Promise.reject(res.status); })
      .then(function (html) {
        var entries = document.getElementById('time-entries');
        if (entries) entries.innerHTML = html;
        modalDirty = true;
      })
      .catch(function () { /* timer still resets below */ });
    window.setTimeout(renderTimer, 2000);
  }

  // ---------- add-task affordance ----------

  function showAddForm(btn) {
    btn.hidden = true;
    var form = btn.closest('.cell').querySelector('.add-task-form');
    form.hidden = false;
    var input = form.querySelector('input[name="name"]');
    if (input) input.focus();
  }

  function hideAddForm(btn) {
    var form = btn.closest('form');
    form.reset();
    form.hidden = true;
    form.closest('.cell').querySelector('.add-task-btn').hidden = false;
  }

  // ---------- column & swimlane management ----------

  function boardId() {
    var main = document.querySelector('main.board');
    return main ? main.dataset.boardId : null;
  }

  function api(path, method, data) {
    return fetch(path, {
      method: method,
      headers: { 'Content-Type': 'application/json' },
      body: data === undefined ? undefined : JSON.stringify(data),
    });
  }

  function alertOnError(res) {
    res.text().then(function (text) {
      window.alert('Request failed (' + res.status + '): ' + text);
    }).catch(function () {
      window.alert('Request failed (' + res.status + ').');
    });
  }

  function colHeaderOf(btn) { return btn.closest('.col-header'); }
  function bandOf(btn) { return btn.closest('.swimlane-band'); }

  function addColumn() {
    var name = window.prompt('New column name:');
    if (!name || !name.trim()) return;
    api('/api/columns', 'POST', { board_id: boardId(), name: name.trim() })
      .then(function (res) { if (res.ok) window.location.reload(); else alertOnError(res); });
  }

  function renameColumn(btn) {
    var h = colHeaderOf(btn);
    var name = window.prompt('Rename column:', h.dataset.columnName);
    if (name === null || !name.trim() || name.trim() === h.dataset.columnName) return;
    api('/api/columns/' + encodeURIComponent(h.dataset.columnId), 'PATCH', { name: name.trim() })
      .then(function (res) { if (res.ok) window.location.reload(); else alertOnError(res); });
  }

  function setWipLimit(btn) {
    var h = colHeaderOf(btn);
    var raw = window.prompt('WIP limit (leave empty to clear):', h.dataset.wipLimit || '');
    if (raw === null) return;
    raw = raw.trim();
    var wip = null;
    if (raw !== '') {
      wip = parseInt(raw, 10);
      if (!wip || wip < 1) { window.alert('Enter a positive number, or leave empty to clear.'); return; }
    }
    api('/api/columns/' + encodeURIComponent(h.dataset.columnId), 'PATCH', { wip_limit: wip })
      .then(function (res) { if (res.ok) window.location.reload(); else alertOnError(res); });
  }

  function moveColumn(btn, dir) {
    var h = colHeaderOf(btn);
    var headers = Array.prototype.slice.call(document.querySelectorAll('.col-header'));
    var index = headers.indexOf(h);
    if (index < 0) return;
    api('/api/columns/' + encodeURIComponent(h.dataset.columnId) + '/move', 'POST', { position: index + dir })
      .then(function (res) { if (res.ok) window.location.reload(); else alertOnError(res); });
  }

  function toggleDoneColumn(btn, isDone) {
    var h = colHeaderOf(btn);
    api('/api/columns/' + encodeURIComponent(h.dataset.columnId), 'PATCH', { is_done: isDone })
      .then(function (res) { if (res.ok) window.location.reload(); else alertOnError(res); });
  }

  function deleteColumn(btn) {
    var h = colHeaderOf(btn);
    var count = parseInt(h.dataset.taskCount, 10) || 0;
    var msg = count > 0
      ? 'Delete column "' + h.dataset.columnName + '"? It still holds ' + count +
        ' task(s) — the server will refuse until they are moved or deleted.'
      : 'Delete column "' + h.dataset.columnName + '"?';
    if (!window.confirm(msg)) return;
    api('/api/columns/' + encodeURIComponent(h.dataset.columnId), 'DELETE')
      .then(function (res) { if (res.ok) window.location.reload(); else alertOnError(res); });
  }

  function addSwimlane() {
    var name = window.prompt('New swimlane name:');
    if (!name || !name.trim()) return;
    api('/api/swimlanes', 'POST', { board_id: boardId(), name: name.trim() })
      .then(function (res) { if (res.ok) window.location.reload(); else alertOnError(res); });
  }

  function renameSwimlane(btn) {
    var band = bandOf(btn);
    var name = window.prompt('Rename swimlane:', band.dataset.swimlaneName);
    if (name === null || !name.trim() || name.trim() === band.dataset.swimlaneName) return;
    api('/api/swimlanes/' + encodeURIComponent(band.dataset.swimlaneId), 'PATCH', { name: name.trim() })
      .then(function (res) { if (res.ok) window.location.reload(); else alertOnError(res); });
  }

  function moveSwimlane(btn, dir) {
    var band = bandOf(btn);
    var bands = Array.prototype.slice.call(document.querySelectorAll('.swimlane-band'));
    var index = bands.indexOf(band);
    if (index < 0) return;
    api('/api/swimlanes/' + encodeURIComponent(band.dataset.swimlaneId) + '/move', 'POST', { position: index + dir })
      .then(function (res) { if (res.ok) window.location.reload(); else alertOnError(res); });
  }

  function deleteSwimlane(btn) {
    var band = bandOf(btn);
    var count = parseInt(band.dataset.taskCount, 10) || 0;
    var msg = count > 0
      ? 'Delete swimlane "' + band.dataset.swimlaneName + '"? It still holds ' + count +
        ' task(s) — the server will refuse until they are moved or deleted.'
      : 'Delete swimlane "' + band.dataset.swimlaneName + '"?';
    if (!window.confirm(msg)) return;
    api('/api/swimlanes/' + encodeURIComponent(band.dataset.swimlaneId), 'DELETE')
      .then(function (res) { if (res.ok) window.location.reload(); else alertOnError(res); });
  }

  // ---------- boot ----------

  // Card clicks open the modal (but not while dragging, and not from
  // interactive elements inside a card).
  document.addEventListener('click', function (e) {
    if (dragging) return;
    if (e.target.closest('button, a, input, select, textarea, form, .modal')) return;
    var card = e.target.closest('.task-card');
    if (card && card.dataset.taskId) openModal(card.dataset.taskId);
  });

  // Keyboard: Enter on a focused card opens it too.
  document.addEventListener('keydown', function (e) {
    if (e.key !== 'Enter') return;
    var active = document.activeElement;
    if (active && active.classList && active.classList.contains('task-card')) {
      openModal(active.dataset.taskId);
    }
  });

  document.addEventListener('DOMContentLoaded', initSortable);
  if (document.querySelector('.task-list')) initSortable(); // in case DOMContentLoaded already fired

  // Called from inline onclick handlers in the templates.
  window.showAddForm = showAddForm;
  window.hideAddForm = hideAddForm;
  window.closeModal = closeModal;
  window.saveModalTask = saveModalTask;
  window.deleteModalTask = deleteModalTask;
  window.addColumn = addColumn;
  window.renameColumn = renameColumn;
  window.setWipLimit = setWipLimit;
  window.moveColumn = moveColumn;
  window.toggleDoneColumn = toggleDoneColumn;
  window.deleteColumn = deleteColumn;
  window.addSwimlane = addSwimlane;
  window.renameSwimlane = renameSwimlane;
  window.moveSwimlane = moveSwimlane;
  window.deleteSwimlane = deleteSwimlane;
})();
