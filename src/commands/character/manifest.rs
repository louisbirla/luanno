use crate::{
	helpers::db::data_db,
	models::{entity::Entity, player::Player},
};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
/// Create a new character!
pub async fn manifest(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let db = data_db(&data);

	// Find the Player struct to attach a new entity
	let mut player = Player::from_user_id(db, msg.author.id).await?;

	if player.entity.is_some() {
		msg.reply(&ctx.http, "You already have a character.")
			.await?;
		return Ok(());
	};

	// Create the default character entity
	let entity = Entity::default_character(db).await?.insert(db).await?;
	// Attach the entity to the player
	player.update_entity(db, Some(entity)).await?;

	msg.reply(&ctx.http, "Character created! Welcome to the world.")
		.await?;

	Ok(())
}
