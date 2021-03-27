use crate::{
	helpers::{db::data_db, name::user_name},
	models::history::status::GameStatus,
};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
/// Displays information about the current game
pub async fn now(ctx: &Context, msg: &Message) -> CommandResult {
	let data = ctx.data.read().await;
	let db = data_db(&data);

	let status = GameStatus::new(db).await?;
	let mut who = "*NPC's turn*".to_string();

	if let Some(player) = status.player {
		let user = player.user(&ctx.http).await?;
		let name = user_name(&ctx.http, user).await;
		who = format!(
			"__{}__'s turn ({}/{} ap left)",
			name, status.actor.ap, status.actor.max_ap
		);
	}

	msg.reply(
		&ctx.http,
		format!(
			"**Game Status**
{}
Round: {}
Turn: {}/{}",
			who, status.round.id, status.turn.id, status.turn_count
		),
	)
	.await?;

	Ok(())
}
