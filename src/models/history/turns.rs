use super::turn::Turn;
use mongodb::{error::Error, Database};

use super::History;

impl History {
	/// Returns a vector of all turns ever
	pub async fn turns(db: &Database) -> Result<Vec<Turn>, Error> {
		Ok(History::collection(db)
			.distinct("turn", None, None)
			.await?
			.into_iter()
			.map(|b| Turn::new(b.as_i64().unwrap().into()))
			.collect())
	}
	/// Returns the number of turns ever
	pub async fn turn_count(db: &Database) -> Result<usize, Error> {
		Ok(History::collection(db)
			.distinct("turn", None, None)
			.await?
			.len())
	}
}
