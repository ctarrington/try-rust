CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
CREATE TABLE "measurements" (
    measured_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    recorded_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    measurement_uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
    sensor_uuid UUID NOT NULL,
    object_uuid UUID NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    object_length REAL NOT NULL
);
SELECT create_hypertable('measurements', by_range('measured_at', INTERVAL '60 minutes'));
