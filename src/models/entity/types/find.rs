use mongodb::{
	bson::{doc, oid::ObjectId, Document},
	error::Error,
	Database,
};

use crate::consts::ENTITY_TYPE_COLLECTION_NAME;

use super::EntityType;

impl EntityType {
	/// Finds an entity type by it's ID
	pub async fn from_id(db: &Database, id: &ObjectId) -> Result<Option<Self>, Error> {
		let filter = doc! { "_id": id };

		Self::find_one(db, filter).await
	}

	/// Finds an entity type by it's name
	pub async fn from_name(db: &Database, name: &str) -> Result<Option<Self>, Error> {
		let filter = doc! { "name": name };

		Self::find_one(db, filter).await
	}
}

impl EntityType {
	/// Finds an entity type from the filter provided
	pub async fn find_one(db: &Database, filter: Document) -> Result<Option<Self>, Error> {
		let entity_type = db
			.collection(ENTITY_TYPE_COLLECTION_NAME)
			.find_one(filter, None)
			.await?;
		let entity_type: Option<Self> = entity_type.map(Self::from);

		Ok(entity_type)
	}
}
