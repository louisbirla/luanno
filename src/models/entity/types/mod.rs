use mongodb::{
	bson::{doc, oid::ObjectId, Bson, Document},
	error::Error,
	Database,
};

use crate::consts::ENTITY_TYPE_COLLECTION_NAME;

pub struct EntityType {
	pub name: String,
}

impl EntityType {
	pub async fn from_id(db: &Database, id: &ObjectId) -> Result<Self, Error> {
		let entity_type = EntityType::from_id_optional(db, id).await?;

		Ok(entity_type.unwrap())
	}

	pub async fn from_id_optional(db: &Database, id: &ObjectId) -> Result<Option<Self>, Error> {
		let type_collection = db.collection(ENTITY_TYPE_COLLECTION_NAME);

		let filter = doc! { "_id": id };
		let entity_type = type_collection.find_one(filter, None).await?;
		let entity_type: Option<Self> = entity_type.map(|doc| doc.into());

		Ok(entity_type)
	}
}

impl From<Document> for EntityType {
	fn from(doc: Document) -> Self {
		let name = doc.get("name").and_then(Bson::as_str).unwrap();

		Self {
			name: name.to_string(),
		}
	}
}

impl From<EntityType> for Document {
	fn from(entity: EntityType) -> Document {
		let mut doc = doc! {};

		doc.insert("name", entity.name);

		doc
	}
}
