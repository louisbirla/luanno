use crate::{
	helpers::{db::data_db, name::user_name},
	models::player::Player,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
	framework::standard::{macros::command, Args, CommandResult},
	utils::parse_mention,
};

#[command]
/// Show your character status
pub async fn status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let data = ctx.data.read().await;
	let db = data_db(&data);

	let mut user = msg.author.clone();

	// If a user is mentioned, show their status instead of yours
	let mention = args.rest();
	if !mention.trim().is_empty() {
		user = UserId(parse_mention(mention).expect("Mention was invalid"))
			.to_user(&ctx.http)
			.await?;
	}

	let player = Player::from_user_id(db, user.id).await?;
	// Make sure the user has a character
	let entity = if let Some(entity) = player.entity {
		entity
	} else {
		msg.reply(&ctx.http, "You don't have a character yet.")
			.await?;
		return Ok(());
	};

	let name = user_name(&ctx.http, user).await;
	msg.channel_id
		.send_message(&ctx.http, |m| {
			m.embed(|e| {
				e.title(format!("{}/status", name));
				e.field(
					"AP :crossed_swords:",
					format!("{}/{}", entity.ap, entity.max_ap),
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
