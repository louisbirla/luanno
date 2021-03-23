use crate::{helpers::db::data_db, models::player::Player};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

mod gum_hourly;
use gum_hourly::*;
mod gum_flip;
use gum_flip::*;

#[command]
#[description = "Display your gum balance."]
#[aliases("mygum", "my_gum")]
#[sub_commands(hourly, flip)]
pub async fn gum(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let db = data_db(&data);

	let player = Player::from_user_id(db, msg.author.id).await.unwrap();

	msg.reply(&ctx.http, format!("You have **{}** gum.", player.gum))
		.await
		.unwrap();

	Ok(())
}
