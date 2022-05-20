use std::sync::Arc;
use serenity::prelude::TypeMapKey;
use sqlx::SqlitePool;

pub struct DBConnection;

impl TypeMapKey for DBConnection {
    type Value = Arc<SqlitePool>;
}