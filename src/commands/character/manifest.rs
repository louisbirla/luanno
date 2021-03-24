use crate::{
	helpers::db::data_db,
	models::{entity::Entity, player::Player},
};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Create a new character!"]
pub async fn manifest(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let db = data_db(&data);

	let mut player = Player::from_user_id(db, msg.author.id).await?;
	if player.entity.is_none() {
		let entity = Entity::character_default(db).await?;
		let entity = entity.insert(db).await?;
		player.update_entity(db, Some(entity)).await?;
		msg.reply(&ctx.http, "Character created! Welcome to the world.")
			.await?;
	} else {
		msg.reply(&ctx.http, "You already have a character.")
			.await?;
	};

	Ok(())
}
