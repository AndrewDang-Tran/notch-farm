use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use std::{fmt, str::FromStr};
use serde::de::StdError;
use serenity::model::id::{GuildId, UserId};


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

impl StdError for ArgumentStatusParseError {}
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
    pub guild_id: i64,
    pub argument_starter_id: i64,
    pub dissenter_id: i64,
    pub description: String,
    pub status: String,
    pub notch_taker_id: Option<i64>
}

#[derive(Serialize)]
pub struct Argument {
    pub argument_id: i64,
    pub guild_id: i64,
    pub argument_starter_id: i64,
    pub dissenter_id: i64,
    pub description: String,
    pub status: ArgumentStatus,
    pub notch_taker_id: Option<i64>
}

impl Argument {
    pub(crate) fn from_db(item: DBArgument) -> Result<Argument, ArgumentStatusParseError> {
        let status_result = ArgumentStatus::from_str(&item.status);
        Ok(Argument {
            argument_id: item.argument_id,
            guild_id: item.guild_id,
            argument_starter_id: item.argument_starter_id,
            dissenter_id: item.dissenter_id,
            description: item.description,
            status: status_result?,
            notch_taker_id: item.notch_taker_id
        })
    }
}

impl<'a> FromRow<'a, SqliteRow> for DBArgument {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(DBArgument {
            argument_id: row.get(0),
            guild_id: row.get(1),
            argument_starter_id: row.get(2),
            dissenter_id: row.get(3),
            description: row.get(4),
            status: row.get(5),
            notch_taker_id: row.get(6)
        })
    }
}

pub struct CreateArgumentParams {
    pub guild_id: GuildId,
    pub argument_starter_id: UserId,
    pub dissenter_id: UserId,
    pub description: String
}

pub struct UpdateNotchTakerParams {
    pub argument_id: i64,
    pub notch_taker: UserId,
}
