mod actor;
mod current;
mod id;
pub use id::TurnId;

pub struct Turn {
	pub id: TurnId,
}

impl Turn {
	pub fn new(id: TurnId) -> Self {
		Self { id }
	}
}
