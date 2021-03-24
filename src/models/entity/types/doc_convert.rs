use mongodb::bson::{doc, Bson, Document};

use super::EntityType;

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
