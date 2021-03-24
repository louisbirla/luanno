use mongodb::{
	bson::{doc, Bson, Document},
	error::Error,
	Database,
};
use serenity::model::id::UserId;

use crate::{consts::PLAYER_INITIAL_GUM, models::entity::Entity};

use super::Player;

impl Player {
	/// Constructs a Player from its document. Uses the DB for finding the Entity data
	pub async fn from_doc(db: &Database, doc: Document) -> Result<Self, Error> {
		let user_id = UserId(doc.get("user_id").and_then(Bson::as_i64).unwrap() as u64);
		let gum = doc
			.get("gum")
			.and_then(Bson::as_i64)
			.unwrap_or(PLAYER_INITIAL_GUM);
		let entity_id = doc.get("entity").and_then(Bson::as_object_id);

		let mut entity: Option<Entity> = None;
		if let Some(id) = entity_id {
			entity = Some(Entity::from_id(db, id).await?);
		}

		Ok(Self {
			user_id,
			gum,
			entity,
		})
	}
}

impl From<Player> for Document {
	fn from(player: Player) -> Self {
		Document::from(&player)
	}
}

impl From<&Player> for Document {
	fn from(player: &Player) -> Self {
		let mut doc = doc! {};

		doc.insert("user_id", player.user_id.as_u64());
		doc.insert("gum", player.gum);

		if let Some(entity) = &player.entity {
			doc.insert("entity", entity.id.clone());
		}

		doc
	}
}
