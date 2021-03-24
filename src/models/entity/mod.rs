use crate::consts::ENTITY_COLLECTION_NAME;
use mongodb::{
	bson::{doc, oid::ObjectId, Bson, Document},
	error::Error,
	Database,
};

use self::types::EntityType;
pub mod character_default;
mod types;

pub struct Entity {
	pub id: ObjectId,
	pub mana: i32,
	pub max_mana: i32,
	pub health: i32,
	pub max_health: i32,
	pub action: i32,
	pub max_action: i32,
	pub entity_type: EntityType,
}

impl Entity {
	pub async fn from_id(db: &Database, id: &ObjectId) -> Result<Self, Error> {
		let entity_type = Self::from_id_optional(db, id).await?;

		Ok(entity_type.expect("Entity didn't exist"))
	}

	pub async fn from_id_optional(db: &Database, id: &ObjectId) -> Result<Option<Self>, Error> {
		let type_collection = db.collection(ENTITY_COLLECTION_NAME);

		let filter = doc! { "_id": id };
		let entity_doc = type_collection.find_one(filter, None).await?;
		let mut entity_type: Option<Self> = None;

		if let Some(doc) = entity_doc {
			entity_type = Some(Self::from_doc(db, doc).await?);
		}

		Ok(entity_type)
	}
}

impl Entity {
	pub async fn from_doc(db: &Database, doc: Document) -> Result<Self, Error> {
		let mana = doc.get("mana").and_then(Bson::as_i32).unwrap();
		let max_mana = doc.get("max_mana").and_then(Bson::as_i32).unwrap();
		let health = doc.get("health").and_then(Bson::as_i32).unwrap();
		let max_health = doc.get("max_health").and_then(Bson::as_i32).unwrap();
		let action = doc.get("action_points").and_then(Bson::as_i32).unwrap();
		let max_action = doc.get("max_action_points").and_then(Bson::as_i32).unwrap();

		let type_id = doc.get("type").and_then(Bson::as_object_id).unwrap();
		let entity_type = EntityType::from_id(db, type_id).await?.unwrap();

		let id = doc.get("_id").and_then(Bson::as_object_id).unwrap();

		Ok(Self {
			mana,
			max_mana,
			health,
			max_health,
			action,
			max_action,
			entity_type,
			id: id.clone(),
		})
	}
}

impl Entity {
	pub async fn insert(self, db: &Database) -> Result<Self, Error> {
		let mut entity = self;
		let entity_collection = db.collection(ENTITY_COLLECTION_NAME);
		let doc = Document::from(&entity);

		let res = entity_collection.insert_one(doc, None).await?;

		entity.id = res.inserted_id.as_object_id().unwrap().clone();

		Ok(entity)
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
		doc.insert("action_points", entity.action);
		doc.insert("max_action_points", entity.max_action);

		doc.insert("type", entity.entity_type.id.clone());

		doc
	}
}
