use mongodb::bson::{doc, Bson, Document};

use crate::models::actions::ActionName;

use super::summary::ActionSummary;

pub struct ActionReport {
	pub name: ActionName,
	pub summary: ActionSummary,
}

impl From<&Document> for ActionReport {
	fn from(doc: &Document) -> Self {
		let name = doc
			.get("name")
			.and_then(Bson::as_str)
			.map(ActionName::from)
			.unwrap();
		let summary = doc
			.get("summary")
			.and_then(Bson::as_document)
			.map(ActionSummary::from)
			.unwrap_or_default();

		Self { name, summary }
	}
}

impl From<Document> for ActionReport {
	fn from(doc: Document) -> Self {
		Self::from(&doc)
	}
}

impl From<ActionReport> for Document {
	fn from(report: ActionReport) -> Self {
		let mut doc = doc! {};

		doc.insert("name", report.name.to_string());
		doc.insert("summary", Document::from(report.summary));

		doc
	}
}
