use super::Round;

#[derive(Clone)]
pub struct RoundId(i64);

impl From<i64> for RoundId {
	fn from(num: i64) -> Self {
		RoundId(num)
	}
}

impl From<RoundId> for i64 {
	fn from(id: RoundId) -> Self {
		id.0
	}
}

impl From<RoundId> for Round {
	fn from(id: RoundId) -> Self {
		Round::new(id)
	}
}

impl From<Round> for RoundId {
	fn from(round: Round) -> Self {
		round.id
	}
}

impl std::fmt::Display for RoundId {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
