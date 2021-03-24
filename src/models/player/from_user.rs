use mongodb::{bson::doc, error::Error, Database};
use serenity::model::id::UserId;

use super::Player;

impl Player {
	/// Find a Player from a user_id. If no Player exists for the user, one will be created.
	pub async fn from_user_id(db: &Database, user_id: UserId) -> Result<Self, Error> {
		let mut player = Player::from_user_id_optional(db, user_id).await?;

		if player.is_none() {
			// Make a new player
			player = Some(Player::new(user_id).insert(db).await?);
		}

		Ok(player.expect("Creating a player still didn't work"))
	}

	/// Tries to find a Player that has a matching user id
	pub async fn from_user_id_optional(
		db: &Database,
		user_id: UserId,
	) -> Result<Option<Self>, Error> {
		let filter = doc! { "user_id": user_id.as_u64() };
		let doc = Player::collection(db).find_one(filter, None).await?;

		let mut player: Option<Self> = None;
		if let Some(doc) = doc {
			player = Some(Self::from_doc(db, doc).await?);
		}

		Ok(player)
	}
}
