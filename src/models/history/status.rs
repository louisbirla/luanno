use mongodb::{error::Error, Database};

use crate::models::{entity::Entity, player::Player};

use super::{round::Round, turn::Turn};

pub struct GameStatus {
	pub round: Round,
	pub turn: Turn,
	pub turn_count: usize,
	pub turns_passed: usize,
	pub turns_left: usize,
	pub actor: Entity,
	pub player: Option<Player>,
}

impl GameStatus {
	pub async fn new(db: &Database) -> Result<Self, Error> {
		let round = Round::current(db).await?;
		let turn = Turn::current(db).await?;
		let actor = turn.actor(db).await?;
		let player = actor.player(db).await?;
		Ok(Self {
			turn_count: round.turn_count(db).await?,
			turns_passed: round.turns_passed(db).await?,
			turns_left: round.turns_left(db).await?,
			actor,
			round,
			turn,
			player,
		})
	}
}
