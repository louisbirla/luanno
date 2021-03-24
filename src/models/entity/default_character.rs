use mongodb::{bson::oid::ObjectId, error::Error, Database};

use super::{types::EntityType, Entity};

impl Entity {
	/// The default character entity
	pub async fn default_character(db: &Database) -> Result<Self, Error> {
		let mana = 20;
		let health = 20;
		let action = 10;

		let entity = Entity {
			mana,
			max_mana: mana,
			health,
			max_health: health,
			action,
			max_action: action,
			entity_type: EntityType::from_name(db, "Character")
				.await?
				.expect("Character entity needs to exist in the database"),
			id: ObjectId::new(),
		};

		Ok(entity)
	}
}
