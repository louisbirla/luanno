use mongodb::{bson::doc, error::Error, options::FindOneOptions, Database};

use super::{super::Entity, super::History, Turn};

impl Turn {
	/// Returns the entity who this turn belongs to
	pub async fn actor(&self, db: &Database) -> Result<Entity, Error> {
		// Get a document with this turn. It may not exist because this may
		// be a future turn.
		let turn_doc = History::collection(db)
			.find_one(doc! { "turn": i64::from(self.id.clone()) }, None)
			.await?;
		// If it exists, there is a definite actor for it
		if let Some(doc) = turn_doc {
			let actor_id = doc.get_object_id("actor").unwrap();
			Entity::from_id(db, actor_id).await
		// If it doesn't exist, the next entity with AP to go should be the actor
		// for this round.
		} else {
			let mut options = FindOneOptions::default();
			options.sort = Some(doc! { "round_turn": 1 });
			let entity_doc = Entity::collection(db)
				.find_one(doc! { "action_points": { "$ne": 0 } }, options)
				.await?
				.unwrap();
			Entity::from_doc(db, entity_doc).await
		}
	}
}
