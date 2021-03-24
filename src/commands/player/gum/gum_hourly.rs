use crate::{helpers::db::data_db, models::player::Player};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

const HOURLY_GUM: i64 = 10;

#[command]
#[bucket = "hourly"]
#[description = "Claim free gum hourly."]
pub async fn hourly(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let db = data_db(&data);

	let mut player = Player::from_user_id(db, msg.author.id).await?;

	player.update_gum(db, player.gum + HOURLY_GUM).await?;

	msg.reply(
		&ctx.http,
		format!("**+{} gum** = {} gum", HOURLY_GUM, player.gum),
	)
	.await?;

	Ok(())
}
