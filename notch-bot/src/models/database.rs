use std::sync::Arc;
use serenity::prelude::TypeMapKey;
use sqlx::SqlitePool;
use tokio::sync::RwLock;

pub struct DBConnection;

impl TypeMapKey for DBConnection {
    type Value = Arc<RwLock<SqlitePool>>;
}