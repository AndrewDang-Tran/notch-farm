
pub enum ArgumentStatus {
    InProgress,
    NotchTaken,
}

impl ArgumentStatus {
    fn as_str(&self) -> &'static str {
        match self {
            ArgumentStatus::InProgress => "InProgress",
            ArgumentStatus::NotchTaken => "NotchTaken"
        }
    }
}

pub struct Argument {
    pub argument_id: u64,
    pub group_id: u64,
    pub argument_starter: u64,
    pub dissenter: u64,
    pub description: String,
    pub status: ArgumentStatus
}