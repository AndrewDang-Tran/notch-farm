use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Encode, Row};
use std::{error, fmt, str::FromStr};
use sqlx::database::HasArguments;
use sqlx::encode::IsNull;


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

#[derive(Debug, Clone)]
pub struct ArgumentStatusParseError;

impl fmt::Display for ArgumentStatusParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to deserialize ArgumentStatus from string")
    }
}

impl FromStr for ArgumentStatus {
    type Err = ArgumentStatusParseError;

    fn from_str(s: &str)-> Result<Self, Self::Err> {
        match s {
            "InProgress" => Ok(ArgumentStatus::InProgress),
            "NotchTaken" => Ok(ArgumentStatus::NotchTaken),
            _ => Err(ArgumentStatusParseError)
        }
    }
}

pub struct DBArgument {
    pub argument_id: i64,
    pub group_id: i64,
    pub argument_starter: i64,
    pub dissenter: i64,
    pub description: String,
    pub status: String,
    pub notch_taker: Option<i64>
}

#[derive(Serialize)]
pub struct Argument {
    pub argument_id: i64,
    pub group_id: i64,
    pub argument_starter: i64,
    pub dissenter: i64,
    pub description: String,
    pub status: ArgumentStatus,
    pub notch_taker: Option<i64>
}

impl Argument {
    pub(crate) fn from_db(item: DBArgument) -> Result<Argument, ArgumentStatusParseError> {
        let status_result = ArgumentStatus::from_str(&item.status);
        Ok(Argument {
            argument_id: item.argument_id,
            group_id: item.group_id,
            argument_starter: item.argument_starter,
            dissenter: item.dissenter,
            description: item.description,
            status: status_result?,
            notch_taker: item.notch_taker
        })
    }
}

impl<'a> FromRow<'a, SqliteRow> for DBArgument {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(DBArgument {
            argument_id: row.get(0),
            group_id: row.get(1),
            argument_starter: row.get(2),
            dissenter: row.get(3),
            description: row.get(4),
            status: row.get(5),
            notch_taker: row.get(6)
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateArgumentRequest {
    pub group_id: i64,
    pub argument_starter: i64,
    pub dissenter: i64,
    pub description: String
}


#[derive(Deserialize)]
pub struct GetArgumentsParams {
    pub group_id: i64
}
