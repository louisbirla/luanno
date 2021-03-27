use mongodb::bson::{doc, Bson, Document};

/// Summary for the effect of the action
pub struct ActionSummary {
	/// The summary logged by an omniscient viewpoint
	pub god_log: Option<String>,
	/// The summary as shown to whoever did the action
	pub actor_log: Option<String>,
	/// The summary as shown to the public
	pub public_log: Option<String>,
}

impl Default for ActionSummary {
	fn default() -> Self {
		Self {
			god_log: None,
			actor_log: None,
			public_log: None,
		}
	}
}

impl From<&Document> for ActionSummary {
	fn from(doc: &Document) -> Self {
		let god_log = doc.get("god_log").and_then(Bson::as_str).map(String::from);
		let actor_log = doc
			.get("actor_log")
			.and_then(Bson::as_str)
			.map(String::from);
		let public_log = doc
			.get("public_log")
			.and_then(Bson::as_str)
			.map(String::from);

		Self {
			god_log,
			actor_log,
			public_log,
		}
	}
}

impl From<Document> for ActionSummary {
	fn from(doc: Document) -> Self {
		Self::from(&doc)
	}
}

impl From<ActionSummary> for Document {
	fn from(summary: ActionSummary) -> Self {
		let mut doc = doc! {};

		if let Some(log) = summary.god_log {
			doc.insert("god_log", log);
		}
		if let Some(log) = summary.actor_log {
			doc.insert("actor_log", log);
		}
		if let Some(log) = summary.public_log {
			doc.insert("public_log", log);
		}

		doc
	}
}
