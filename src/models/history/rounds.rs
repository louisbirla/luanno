use super::round::Round;
use mongodb::{error::Error, Database};

use super::History;

impl History {
	/// Returns a vector of all turns in the round
	pub async fn rounds(db: &Database) -> Result<Vec<Round>, Error> {
		Ok(History::collection(db)
			.distinct("round", None, None)
			.await?
			.into_iter()
			.map(|b| Round::new(b.as_i64().unwrap().into()))
			.collect())
	}
	/// Returns the number of turns in the round
	pub async fn round_count(db: &Database) -> Result<usize, Error> {
		Ok(History::collection(db)
			.distinct("round", None, None)
			.await?
			.len())
	}
}
