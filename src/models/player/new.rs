use crate::consts::{PLAYER_COLLECTION_NAME, PLAYER_INITIAL_GUM};
use mongodb::{bson::Document, error::Error, Collection, Database};
use serenity::model::id::UserId;

use super::Player;

impl Player {
	/// Create a Player for a user using the defaults
	pub fn new(user_id: UserId) -> Self {
		Player {
			user_id,
			gum: PLAYER_INITIAL_GUM,
			entity: None,
		}
	}
}

impl Player {
	/// Inserts a Player into the database, which returns an accurate version
	/// of the player
	pub async fn insert(self, db: &Database) -> Result<Self, Error> {
		Player::collection(db)
			.insert_one(Document::from(&self), None)
			.await?;

		Ok(self)
	}
}

impl Player {
	/// A handy function for accessing the Player collection
	pub fn collection(db: &Database) -> Collection<Document> {
		db.collection(PLAYER_COLLECTION_NAME)
	}
}
