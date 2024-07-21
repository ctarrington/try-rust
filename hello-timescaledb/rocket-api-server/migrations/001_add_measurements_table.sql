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
    altitude REAL NOT NULL,
    x_position REAL NOT NULL,
    y_position REAL NOT NULL,
    z_position REAL NOT NULL,
    x_velocity REAL NOT NULL,
    y_velocity REAL NOT NULL,
    z_velocity REAL NOT NULL,
    object_length REAL,
    object_width REAL,
    object_height REAL,
    flavor TEXT,
    toppings TEXT,
    color TEXT,
    texture TEXT
);
SELECT create_hypertable('measurements', by_range('measured_at', INTERVAL '120 minutes'));
