CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
CREATE TABLE "measurements" (
    measured_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    measurement_id SERIAL,
    object_id UUID NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    object_length REAL NOT NULL
);
SELECT create_hypertable('measurements', 'measured_at');
