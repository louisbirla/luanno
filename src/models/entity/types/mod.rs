use mongodb::{
	bson::{doc, oid::ObjectId, Bson, Document},
	error::Error,
	Database,
};

use crate::consts::ENTITY_TYPE_COLLECTION_NAME;

pub struct EntityType {
	pub id: ObjectId,
	pub name: String,
}

impl EntityType {
	pub async fn from_id(db: &Database, id: &ObjectId) -> Result<Option<Self>, Error> {
		let filter = doc! { "_id": id };

		let result = Self::find_one(db, filter).await?;

		Ok(result)
	}

	pub async fn from_name(db: &Database, name: &str) -> Result<Option<Self>, Error> {
		let filter = doc! { "name": name };

		let result = Self::find_one(db, filter).await?;

		Ok(result)
	}
}

impl EntityType {
	pub async fn find_one(db: &Database, filter: Document) -> Result<Option<Self>, Error> {
		let type_collection = db.collection(ENTITY_TYPE_COLLECTION_NAME);

		let entity_type = type_collection.find_one(filter, None).await?;
		let entity_type: Option<Self> = entity_type.map(|doc| doc.into());

		Ok(entity_type)
	}
}

impl From<Document> for EntityType {
	fn from(doc: Document) -> Self {
		let name = doc.get("name").and_then(Bson::as_str).unwrap();
		let id = doc.get("_id").and_then(Bson::as_object_id).unwrap();

		Self {
			name: name.to_string(),
			id: id.clone(),
		}
	}
}

impl From<EntityType> for Document {
	fn from(entity: EntityType) -> Document {
		let mut doc = doc! {};

		doc.insert("name", entity.name);
		doc.insert("_id", entity.id);

		doc
	}
}
