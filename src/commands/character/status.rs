use crate::{helpers::db::data_db, models::player::Player};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
/// Show your character status
pub async fn status(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let db = data_db(&data);

	let player = Player::from_user_id(db, msg.author.id).await?;
	// Make sure the user has a character
	let entity = if let Some(entity) = player.entity {
		entity
	} else {
		msg.reply(&ctx.http, "You don't have a character yet.")
			.await?;
		return Ok(());
	};

	msg.channel_id
		.send_message(&ctx.http, |m| {
			m.embed(|e| {
				e.title(format!("{}/status", msg.author.name));
				e.field(
					"AP :crossed_swords:",
					format!("{}/{}", entity.action, entity.max_action),
					true,
				);
				e.field(
					"Health :heart:",
					format!("{}/{}", entity.health, entity.max_health),
					true,
				);
				e.field(
					"Mana :zap:",
					format!("{}/{}", entity.mana, entity.max_mana),
					true,
				);

				e
			});
			m
		})
		.await?;

	Ok(())
}
