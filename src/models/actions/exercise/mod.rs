use mongodb::{bson::Document, error::Error, Database};
use serenity::async_trait;

use super::{Action, ActionName};

mod doc_convert;

struct Exercise {
	pub ap: i32,
	pub mana_use: i64,

	pub nothing_infl: i32,
	pub max_mana_infl: i32,
	pub max_health_infl: i32,

	pub max_mana_bump: i64,
	pub max_health_bump: i64,
}

impl Default for Exercise {
	fn default() -> Self {
		Self {
			ap: 10,
			mana_use: 1,
			nothing_infl: 1,
			max_mana_infl: 0,
			max_health_infl: 0,
			max_mana_bump: 1,
			max_health_bump: 1,
		}
	}
}

#[async_trait]
impl Action for Exercise {
	fn name(&self) -> ActionName {
		"exercise".into()
	}

	async fn from_doc(db: &Database, doc: Document) -> Result<Box<Self>, Error> {
		Ok(box Self::handle_from_doc(db, doc).await?)
	}
}
