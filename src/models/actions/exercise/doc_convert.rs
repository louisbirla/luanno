use mongodb::{
	bson::{Bson, Document},
	error::Error,
	Database,
};

use super::Exercise;

impl Exercise {
	/// Conversion from an action document (that is known to be exercise) into
	/// the exercise action struct
	pub async fn handle_from_doc(_db: &Database, doc: Document) -> Result<Self, Error> {
		let mut action = Self::default();

		action.ap = doc.get("ap").and_then(Bson::as_i32).unwrap_or(action.ap);
		let data = match doc.get("data").and_then(Bson::as_document) {
			Some(data) => data,
			None => return Ok(action),
		};

		if let Some(n) = data.get("mana_use").and_then(Bson::as_i64) {
			action.mana_use = n;
		}

		if let Some(n) = data.get("nothing_infl").and_then(Bson::as_i32) {
			action.nothing_infl = n;
		}
		if let Some(n) = data.get("max_mana_infl").and_then(Bson::as_i32) {
			action.max_mana_infl = n;
		}
		if let Some(n) = data.get("max_health_infl").and_then(Bson::as_i32) {
			action.max_health_infl = n;
		}

		if let Some(n) = data.get("max_mana_bump").and_then(Bson::as_i64) {
			action.max_mana_bump = n;
		}
		if let Some(n) = data.get("max_health_bump").and_then(Bson::as_i64) {
			action.max_health_bump = n;
		}

		Ok(action)
	}
}
