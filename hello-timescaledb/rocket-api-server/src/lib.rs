use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Measurement is a single measurement of an object by a Sensor at a time
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Measurement {
    pub measurement_uuid: Option<uuid::Uuid>,
    pub object_uuid: uuid::Uuid,
    pub sensor_uuid: uuid::Uuid,
    pub measured_at: NaiveDateTime,
    pub recorded_at: Option<NaiveDateTime>,
    pub latitude: f32,
    pub longitude: f32,
    pub object_length: f32,
}

/// PathPoint is a single point in a Path as measured by a Sensor
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathPoint {
    pub sensor_uuid: uuid::Uuid,
    pub measured_at: NaiveDateTime,
    pub latitude: f32,
    pub longitude: f32,
}

/// Path is a collection of PathPoints for a single object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Path {
    pub object_uuid: uuid::Uuid,
    pub path_points: Vec<PathPoint>,
}
