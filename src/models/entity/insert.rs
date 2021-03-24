use crate::consts::ENTITY_COLLECTION_NAME;
use mongodb::{bson::Document, error::Error, Collection, Database};

use super::Entity;

impl Entity {
	/// Insert the entity into the database. Returns the correct version of
	/// the entity, with the proper ID given by MongoDB
	pub async fn insert(self, db: &Database) -> Result<Self, Error> {
		let mut entity = self;

		let res = Entity::collection(db)
			.insert_one(Document::from(&entity), None)
			.await?;

		// Correct the entity ID
		entity.id = res.inserted_id.as_object_id().unwrap().clone();

		Ok(entity)
	}
}

impl Entity {
	/// A handy function for accessing the Player collection
	pub fn collection(db: &Database) -> Collection<Document> {
		db.collection(ENTITY_COLLECTION_NAME)
	}
}
