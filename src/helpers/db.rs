use mongodb::Database;
use serenity::prelude::*;
use tokio::sync::RwLockReadGuard;

use crate::DatabaseConnection;

/// Grab the database instance from the data passed by Serenity
pub fn data_db<'a>(data: &'a RwLockReadGuard<'a, TypeMap>) -> &'a Database {
	data.get::<DatabaseConnection>()
		.expect("Database didn't exist in Serenity data")
}
