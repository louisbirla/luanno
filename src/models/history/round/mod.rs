mod current;
mod id;
mod turns;
pub use id::RoundId;

pub struct Round {
	pub id: RoundId,
}

impl Round {
	pub fn new(id: RoundId) -> Self {
		Self { id }
	}
}
