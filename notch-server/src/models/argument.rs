use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ArgumentStatus {
    InProgress,
    NotchTaken,
}

impl ArgumentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArgumentStatus::InProgress => "InProgress",
            ArgumentStatus::NotchTaken => "NotchTaken"
        }
    }
}

pub struct Argument {
    pub argument_id: i64,
    pub group_id: i64,
    pub argument_starter: i64,
    pub dissenter: i64,
    pub description: String,
    pub status: ArgumentStatus,
    pub notch_taker: Option<i64>
}

#[derive(Serialize, Deserialize)]
pub struct CreateArgumentRequest {
    pub group_id: i64,
    pub argument_starter: i64,
    pub dissenter: i64,
    pub description: String
}