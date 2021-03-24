use super::Entity;
use super::Player;
use mongodb::{
	bson::{doc, Document},
	error::Error,
	results::UpdateResult,
	Database,
};

impl Player {
	/// Updates the gum a player has, both for the Player struct and
	/// in the DB
	pub async fn update_gum(
		&mut self,
		db: &Database,
		new_count: i64,
	) -> Result<UpdateResult, Error> {
		self.gum = new_count;

		let update = doc! { "$set": { "gum": new_count }};

		self.update(db, update).await
	}
}

impl Player {
	/// Updates the Player's entity/character. This mutates the Player struct and
	/// also the DB
	pub async fn update_entity(
		&mut self,
		db: &Database,
		entity: Option<Entity>,
	) -> Result<UpdateResult, Error> {
		self.entity = entity;

		// By default remove the entity from the user
		let mut update = doc! { "$unset": { "entity": "" }};

		// If an entity is to be set (not None), then change the update document
		if let Some(entity) = &self.entity {
			update = doc! { "$set": { "entity": &entity.id } }
		}

		self.update(db, update).await
	}
}

impl Player {
	/// Pushes an update to a Player
	pub async fn update(&self, db: &Database, update_doc: Document) -> Result<UpdateResult, Error> {
		let res = Player::collection(db)
			.update_one(doc! {"user_id": self.user_id.as_u64()}, update_doc, None)
			.await?;

		Ok(res)
	}
}
