use crate::{helpers::db::data_db, models::player::Player};
use rand::seq::SliceRandom;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
/// Wager some gum, choose heads or tails, then if you're right you'll get twice the gum back!
pub async fn flip(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	// Parse the inputs
	let wager = args.single::<i64>()?;
	let prediction = args.single::<String>()?;

	let options = vec!["heads".to_string(), "tails".to_string()];
	// Make sure that the prediction is either "heads" or "tails"
	if !options.contains(&prediction) {
		msg.reply(&ctx.http, "You must choose either heads or tails.")
			.await?;
		return Ok(());
	}

	let data = ctx.data.read().await;
	let db = data_db(&data);

	let mut player = Player::from_user_id(db, msg.author.id).await?;

	// Make sure the user has enough to pay the wager
	if player.gum < wager {
		msg.reply(
			&ctx.http,
			format!("You can't wager that much. You have {} gum.", player.gum),
		)
		.await?;
		return Ok(());
	}

	// Randomly choose heads or tails
	let chosen = options
		.choose(&mut rand::thread_rng())
		.expect("There were no options");

	if chosen == &prediction {
		player.update_gum(db, player.gum + wager).await?;
		msg.reply(
			&ctx.http,
			format!("You were correct! **+{} gum** = {} gum", wager, player.gum),
		)
		.await?;
	} else {
		player.update_gum(db, player.gum - wager).await?;
		msg.reply(
			&ctx.http,
			format!(
				"Bummer, it was {}. **-{} gum** = {} gum",
				chosen, wager, player.gum
			),
		)
		.await?;
	}

	Ok(())
}
