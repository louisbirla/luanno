use mongodb::{bson::doc, error::Error, options::FindOneOptions, Database};

use super::{super::History, Turn};

impl Turn {
	/// Returns the current turn
	pub async fn current(db: &Database) -> Result<Self, Error> {
		let mut options = FindOneOptions::default();
		options.sort = Some(doc! { "turn": -1 });
		let doc = History::collection(db)
			.find_one(None, options)
			.await?
			.unwrap();
		let mut turn_id = doc.get_i64("turn").unwrap();
		let mut turn = Self::new(turn_id.into());
		let actor = turn.actor(db).await?;
		if actor.ap == 0 {
			turn_id += 1;
			turn = Self::new(turn_id.into());
		}
		Ok(turn)
	}
}
