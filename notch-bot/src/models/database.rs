use serenity::prelude::TypeMapKey;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct DBConnection;

impl TypeMapKey for DBConnection {
    type Value = Arc<SqlitePool>;
}
