# batch-38 evidence (2026-10-17)

Production 8ec4d30 (code tree equals integ/batch-38 e9a829d): streams Z1
(startup, API and timer fixes) and Z2 (UI fixes).

- `verify-38a-report.md`: blind check of the server/data half, including a
  real-data upgrade audit: a copy of the production database taken before the
  schema-v2 upgrade was upgraded by 9548374 and by the candidate and compared
  table by table (every v1 value kept; row counts identical). `audit/` holds the
  dumper and comparator only, no data.
- `verify-38b-report.md`: blind check of the UI half.

Deploy note: the first switch ran while another session changed the homelab
working copy, so it redeployed the previous ChipFlow pin; the pin was then
committed and pushed (homelab 76493a3) and deployed. The database copy
`/var/lib/chipflow-backups/chipflow-20261017T210555Z-before-8ec4d30.redb` was
taken before the first attempt; production ran the previous build for about
90 seconds between the two switches.
