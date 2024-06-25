use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Measurement {
    pub object_id: uuid::Uuid,
    pub measured_at: NaiveDateTime,
    pub latitude: f32,
    pub longitude: f32,
    pub object_length: f32,
}
