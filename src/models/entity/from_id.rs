use mongodb::{
	bson::{doc, oid::ObjectId},
	error::Error,
	Database,
};

use super::Entity;

impl Entity {
	/// Finds an entity from its ID, and panics if it does not exist
	pub async fn from_id(db: &Database, id: &ObjectId) -> Result<Self, Error> {
		let entity_type = Self::from_id_optional(db, id).await?;

		Ok(entity_type.expect("Entity didn't exist"))
	}

	/// Finds an entity from its ID (if it exists)
	pub async fn from_id_optional(db: &Database, id: &ObjectId) -> Result<Option<Self>, Error> {
		let filter = doc! { "_id": id };

		let entity_doc = Entity::collection(db).find_one(filter, None).await?;

		let mut entity_type: Option<Self> = None;
		if let Some(doc) = entity_doc {
			entity_type = Some(Self::from_doc(db, doc).await?);
		}

		Ok(entity_type)
	}
}
