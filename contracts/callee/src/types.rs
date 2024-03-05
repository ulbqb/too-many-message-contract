use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    pub alpha: u8,
    pub beta: u8,
    pub delta: u8,
    pub gamma: u8,
    pub is_ok: bool,
}
