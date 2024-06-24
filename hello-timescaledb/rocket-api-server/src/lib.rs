use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Measurement {
    pub latitude: f32,
    pub longitude: f32,
    pub object_length: f32,
}
