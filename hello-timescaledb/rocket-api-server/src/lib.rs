use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Measurement {
    pub measurement_id: Option<i32>,
    pub object_uuid: uuid::Uuid,
    pub sensor_uuid: uuid::Uuid,
    pub measured_at: NaiveDateTime,
    pub latitude: f32,
    pub longitude: f32,
    pub object_length: f32,
}
