use serenity::model::id::UserId;

mod doc_convert;
mod new;
mod update;
mod user_convert;

use super::entity::Entity;

/// A player is what represents a user in-game
pub struct Player {
	/// This is the Discord ID of the user
	pub user_id: UserId,
	/// This is how much gum (meta currency) the player has
	pub gum: i64,
	/// This is the entity/character tht the player controls
	pub entity: Option<Entity>,
}
