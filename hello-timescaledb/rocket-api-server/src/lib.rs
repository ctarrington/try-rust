use chrono::{NaiveDateTime, ParseResult};
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Diagnostics {
    pub measured_at: NaiveDateTime,
    pub measurement_count: usize,
    pub object_count: usize,
    pub database_size_gigabytes: f64,
    pub average_measurement_size_bytes: f64,
}

pub fn parse_datetime(datetime_str: &&str) -> ParseResult<NaiveDateTime> {
    chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%dT%H:%M:%S")
}

pub fn convert_to_sqlx_uuid(
    uuid: &uuid::Uuid,
) -> Result<sqlx::types::Uuid, sqlx::types::uuid::Error> {
    sqlx::types::Uuid::parse_str(&uuid.to_string())
}

pub fn convert_to_uuid(uuid: &sqlx::types::Uuid) -> Result<uuid::Uuid, uuid::Error> {
    uuid::Uuid::parse_str(&uuid.to_string())
}
