use crate::models::{
	entity::Entity,
	history::{turn::Turn, History},
};
use mongodb::{
	bson::{doc, Document},
	error::Error,
	Database,
};

use super::Round;

impl Round {
	/// Returns a vector of all turns in this round
	pub async fn turns(&self, db: &Database) -> Result<Vec<Turn>, Error> {
		Ok(History::collection(db)
			.distinct("turn", self.create_filter(), None)
			.await?
			.into_iter()
			.map(|b| Turn::new(b.as_i64().unwrap().into()))
			.collect())
	}
	/// Returns the number of turns in this round (so far)
	pub async fn turns_passed(&self, db: &Database) -> Result<usize, Error> {
		Ok(History::collection(db)
			.distinct("turn", self.create_filter(), None)
			.await?
			.len())
	}
	/// Returns the total number of turns that will be in this round
	pub async fn turn_count(&self, db: &Database) -> Result<usize, Error> {
		Ok(Entity::collection(db).count_documents(None, None).await? as usize)
	}
	/// Returns the number of turns that still need to be done in this round
	pub async fn turns_left(&self, db: &Database) -> Result<usize, Error> {
		// This works because for a turn to end, there cant be any action points left
		Ok(Entity::collection(db)
			.count_documents(doc! { "action_points": { "$ne": 0 } }, None)
			.await? as usize)
	}
}

impl Round {
	/// A useful function to filter history for this round
	pub fn create_filter(&self) -> Document {
		doc! { "round": i64::from(self.id.clone()) }
	}
}
