use mongodb::bson::oid::ObjectId;
mod doc_convert;
mod find;

pub struct EntityType {
	pub id: ObjectId,
	pub name: String,
}
