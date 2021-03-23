use crate::consts::{PLAYER_COLLECTION_NAME, PLAYER_INITIAL_GUM};
use create::create_player;
use mongodb::{
	bson::{doc, Bson, Document},
	error::Error,
	Database,
};
use serenity::model::id::UserId;

use super::entity::Entity;

pub mod create;

pub struct Player {
	pub user_id: UserId,
	pub gum: i64,
	pub entity: Option<Entity>,
}

impl Player {
	pub async fn from_user_id(db: &Database, user_id: UserId) -> Result<Self, Error> {
		let mut player = Player::from_user_id_optional(db, user_id).await?;

		if player.is_none() {
			create_player(db, user_id).await?;
			player = Player::from_user_id_optional(db, user_id).await?;
		}

		Ok(player.unwrap())
	}

	pub async fn from_user_id_optional(
		db: &Database,
		user_id: UserId,
	) -> Result<Option<Self>, Error> {
		let player_collection = db.collection(PLAYER_COLLECTION_NAME);

		let filter = doc! { "user_id": user_id.as_u64() };
		let doc = player_collection.find_one(filter, None).await?;
		let mut player: Option<Self> = None;

		if let Some(doc) = doc {
			player = Some(Self::from_doc(db, doc).await?);
		}

		Ok(player)
	}
}

impl Player {
	pub async fn update_gum(&mut self, db: &Database, new_count: i64) -> Result<(), Error> {
		self.gum = new_count;

		let collection = db.collection(PLAYER_COLLECTION_NAME);
		collection
			.update_one(
				doc! {"user_id": self.user_id.as_u64()},
				doc! { "$set": { "gum": new_count }},
				None,
			)
			.await?;

		Ok(())
	}
}

impl Player {
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
