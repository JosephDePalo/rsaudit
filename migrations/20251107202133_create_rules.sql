CREATE TYPE severity_level AS ENUM (
    'info',
    'low',
    'medium',
    'high',
    'critical'
);

CREATE TYPE check_type AS ENUM (
  'lua'
);

CREATE TABLE rules (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  severity severity_level NOT NULL,
  check_type check_type NOT NULL DEFAULT 'lua',
  script_body TEXT NOT NULL
);

