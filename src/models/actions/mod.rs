use mongodb::{bson::Document, error::Error, Database};
use serenity::async_trait;

mod exercise;

#[async_trait]
pub trait Action {
	fn name(&self) -> ActionName;
	async fn from_doc(db: &Database, doc: Document) -> Result<Box<Self>, Error>;
}

pub struct ActionName(String);

impl From<&str> for ActionName {
	fn from(string: &str) -> Self {
		string.to_string().into()
	}
}

impl From<String> for ActionName {
	fn from(string: String) -> Self {
		ActionName(string)
	}
}

impl ToString for ActionName {
	fn to_string(&self) -> String {
		self.0.clone()
	}
}
