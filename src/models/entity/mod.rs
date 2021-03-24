use self::types::EntityType;
use mongodb::bson::oid::ObjectId;

mod default_character;
mod doc_convert;
mod from_id;
mod insert;
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
