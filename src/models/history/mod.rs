use mongodb::{bson::Document, Collection, Database};

use crate::consts::HISTORY_COLLECTION_NAME;

use self::{action_report::ActionReport, round::RoundId, turn::TurnId};

use super::entity::Entity;
mod action_report;
mod doc_convert;
pub mod round;
mod rounds;
pub mod status;
mod summary;
pub mod turn;
mod turns;

pub struct History {
	pub turn: TurnId,
	pub round: RoundId,
	pub action: ActionReport,
	pub actor: Entity,
}

impl History {
	/// A handy function for accessing the History collection
	pub fn collection(db: &Database) -> Collection<Document> {
		db.collection(HISTORY_COLLECTION_NAME)
	}
}
