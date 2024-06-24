CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
CREATE TABLE "measurements" (
    measured_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    object_id SERIAL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    object_length REAL NOT NULL
);
SELECT create_hypertable('measurements', 'measured_at');
