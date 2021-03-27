use self::types::EntityType;
use mongodb::bson::oid::ObjectId;

mod default_character;
pub mod doc_convert;
mod from_id;
mod insert;
mod player;
mod types;

pub struct Entity {
	pub id: ObjectId,
	pub mana: i32,
	pub max_mana: i32,
	pub health: i32,
	pub max_health: i32,
	pub ap: i32,
	pub max_ap: i32,
	pub entity_type: EntityType,
}
