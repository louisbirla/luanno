use mongodb::{
	bson::{doc, Bson, Document},
	error::Error,
	Database,
};

use crate::models::entity::Entity;

use super::History;

impl History {
	/// Constructs a History from its document. Uses the DB for finding the Entity data
	pub async fn from_doc(db: &Database, doc: Document) -> Result<Self, Error> {
		let turn = doc.get("turn").and_then(Bson::as_i64).unwrap().into();
		let round = doc.get("round").and_then(Bson::as_i64).unwrap().into();
		let report = doc
			.get("action")
			.and_then(Bson::as_document)
			.unwrap()
			.into();
		let entity_id = doc.get("actor").and_then(Bson::as_object_id).unwrap();
		let actor = Entity::from_id(db, entity_id).await?;

		Ok(Self {
			turn,
			round,
			action: report,
			actor,
		})
	}
}

impl From<History> for Document {
	fn from(hist: History) -> Self {
		doc! {
			"turn": i64::from(hist.turn),
			"round": i64::from(hist.round),
			"actor": hist.actor.id,
			"action": Document::from(hist.action),
		}
	}
}
