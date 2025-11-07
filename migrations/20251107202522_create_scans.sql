CREATE TYPE scan_status AS ENUM (
    'pending',
    'running',
    'completed',
    'failed'
);

CREATE TABLE scans (
  id BIGSERIAL PRIMARY KEY,
  device_id BIGINT NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
  status scan_status NOT NULL DEFAULT 'pending'
);
