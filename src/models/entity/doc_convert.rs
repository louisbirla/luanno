use mongodb::{
	bson::{doc, Bson, Document},
	error::Error,
	Database,
};

use super::{types::EntityType, Entity};

impl Entity {
	/// Finds the entity from its document
	pub async fn from_doc(db: &Database, doc: Document) -> Result<Self, Error> {
		// Scalar data
		let mana = doc.get("mana").and_then(Bson::as_i32).unwrap();
		let max_mana = doc.get("max_mana").and_then(Bson::as_i32).unwrap();
		let health = doc.get("health").and_then(Bson::as_i32).unwrap();
		let max_health = doc.get("max_health").and_then(Bson::as_i32).unwrap();
		let action = doc.get("action_points").and_then(Bson::as_i32).unwrap();
		let max_action = doc.get("max_action_points").and_then(Bson::as_i32).unwrap();
		let id = doc.get("_id").and_then(Bson::as_object_id).unwrap();

		// Document has the type's ID, but we need the entity type struct
		let type_id = doc.get("type").and_then(Bson::as_object_id).unwrap();
		let entity_type = EntityType::from_id(db, type_id).await?.unwrap();

		Ok(Self {
			mana,
			max_mana,
			health,
			max_health,
			ap: action,
			max_ap: max_action,
			entity_type,
			id: id.clone(),
		})
	}
}

impl From<Entity> for Document {
	fn from(entity: Entity) -> Self {
		Document::from(&entity)
	}
}

impl From<&Entity> for Document {
	fn from(entity: &Entity) -> Self {
		let mut doc = doc! {};

		doc.insert("mana", entity.mana);
		doc.insert("max_mana", entity.max_mana);
		doc.insert("health", entity.health);
		doc.insert("max_health", entity.max_health);
		doc.insert("action_points", entity.ap);
		doc.insert("max_action_points", entity.max_ap);

		doc.insert("type", entity.entity_type.id.clone());

		doc
	}
}
