use super::Turn;

#[derive(Clone)]
pub struct TurnId(i64);

impl From<i64> for TurnId {
	fn from(num: i64) -> Self {
		TurnId(num)
	}
}

impl From<TurnId> for i64 {
	fn from(id: TurnId) -> Self {
		id.0
	}
}

impl From<TurnId> for Turn {
	fn from(id: TurnId) -> Self {
		Turn::new(id)
	}
}

impl From<Turn> for TurnId {
	fn from(turn: Turn) -> Self {
		turn.id
	}
}

impl std::fmt::Display for TurnId {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
