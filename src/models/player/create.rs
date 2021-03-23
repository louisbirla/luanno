use mongodb::{bson::doc, error::Result, results::InsertOneResult, Database};
use serenity::model::prelude::*;

use crate::consts::{PLAYER_COLLECTION_NAME, PLAYER_INITIAL_GUM};

pub async fn create_player(db: &Database, user_id: UserId) -> Result<InsertOneResult> {
	let player_collection = db.collection(PLAYER_COLLECTION_NAME);

	let player = doc! { "user_id": user_id.as_u64(), "gum": PLAYER_INITIAL_GUM };

	player_collection.insert_one(player, None).await
}
