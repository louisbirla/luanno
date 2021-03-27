use mongodb::{bson::doc, error::Error, Database};

use crate::models::player::Player;

use super::Entity;

impl Entity {
	/// If a player controls this entity, return it
	pub async fn player(&self, db: &Database) -> Result<Option<Player>, Error> {
		let doc = Player::collection(db)
			.find_one(doc! { "entity": self.id.clone() }, None)
			.await?;
		if let Some(doc) = doc {
			Ok(Some(Player::from_doc(db, doc).await?))
		} else {
			Ok(None)
		}
	}
}
