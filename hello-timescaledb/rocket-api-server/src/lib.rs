use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Measurement {
    pub object_id: String,
    pub measured_at: String,
    pub latitude: f32,
    pub longitude: f32,
    pub object_length: f32,
}
