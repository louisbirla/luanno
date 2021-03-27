use mongodb::{bson::doc, error::Error, options::FindOneOptions, Database};

use super::{super::History, Round};

impl Round {
	/// Returns the current round
	pub async fn current(db: &Database) -> Result<Self, Error> {
		let mut options = FindOneOptions::default();
		options.sort = Some(doc! { "round": -1 });
		let doc = History::collection(db)
			.find_one(None, options)
			.await?
			.unwrap();
		let mut round_id = doc.get_i64("round").unwrap();
		let mut round = Self::new(round_id.into());
		if round.turns_left(db).await? == 0 {
			round_id += 1;
			round = Self::new(round_id.into());
		}
		Ok(round)
	}
}
