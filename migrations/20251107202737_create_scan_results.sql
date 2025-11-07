CREATE TYPE check_status AS ENUM (
    'pass',
    'fail',
    'error'
);

CREATE TABLE scan_results (
  id BIGSERIAL PRIMARY KEY,
  scan_id BIGINT NOT NULL REFERENCES scans(id) ON DELETE CASCADE,
  rule_id TEXT NOT NULL REFERENCES rules(id) ON DELETE RESTRICT,
  status check_status NOT NULL,
  details TEXT
);

