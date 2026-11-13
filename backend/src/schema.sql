PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA foreign_keys = ON;
PRAGMA busy_timeout = 5000;
PRAGMA temp_store = MEMORY;

CREATE TABLE IF NOT EXISTS schema_migrations (
  version    INTEGER PRIMARY KEY,
  applied_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS app_settings (
  key        TEXT PRIMARY KEY,
  value      TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS focus_sessions (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  kind        TEXT NOT NULL CHECK (kind IN
              ('pomodoro_focus','pomodoro_break','timer_countdown','timer_countup')),
  planned_sec INTEGER NOT NULL DEFAULT 0,
  actual_sec  INTEGER NOT NULL DEFAULT 0,
  started_at  TEXT NOT NULL,
  ended_at    TEXT,
  local_date  TEXT NOT NULL,
  completed   INTEGER NOT NULL DEFAULT 0,
  skipped     INTEGER NOT NULL DEFAULT 0,
  interrupted INTEGER NOT NULL DEFAULT 0,
  round_index INTEGER
);

CREATE INDEX IF NOT EXISTS idx_focus_date ON focus_sessions(local_date);
CREATE INDEX IF NOT EXISTS idx_focus_kind ON focus_sessions(kind, completed);

CREATE TABLE IF NOT EXISTS todos (
  id           INTEGER PRIMARY KEY AUTOINCREMENT,
  title        TEXT NOT NULL,
  note         TEXT NOT NULL DEFAULT '',
  due_at       TEXT,
  remind_at    TEXT,
  reminded_at  TEXT,
  completed    INTEGER NOT NULL DEFAULT 0,
  created_at   TEXT NOT NULL,
  completed_at TEXT,
  sort_order   INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_todos_open ON todos(completed, due_at);
CREATE INDEX IF NOT EXISTS idx_todos_remind ON todos(completed, remind_at);

CREATE TABLE IF NOT EXISTS reminder_state (
  kind          TEXT PRIMARY KEY CHECK (kind IN ('water','sedentary','eyes')),
  enabled       INTEGER NOT NULL DEFAULT 1,
  interval_sec  INTEGER NOT NULL,
  next_due_at   TEXT,
  last_fired_at TEXT,
  snoozed_until TEXT
);

CREATE TABLE IF NOT EXISTS pet_state (
  id                  INTEGER PRIMARY KEY CHECK (id = 1),
  mood                INTEGER NOT NULL DEFAULT 70,
  energy              INTEGER NOT NULL DEFAULT 80,
  last_interaction_at TEXT,
  last_decay_at       TEXT
);

INSERT OR IGNORE INTO pet_state (id) VALUES (1);

CREATE TABLE IF NOT EXISTS timer_snapshot (
  id         INTEGER PRIMARY KEY CHECK (id = 1),
  payload    TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
