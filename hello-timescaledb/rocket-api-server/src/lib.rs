use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Measurement {
    pub measurement_id: Option<uuid::Uuid>,
    pub object_uuid: uuid::Uuid,
    pub sensor_uuid: uuid::Uuid,
    pub measured_at: NaiveDateTime,
    pub recorded_at: Option<NaiveDateTime>,
    pub latitude: f32,
    pub longitude: f32,
    pub object_length: f32,
}
