use crate::consts::MOD_ROLE_ID;

use super::*;
use serenity::framework::standard::{macros::command, Args};
use serenity::{framework::standard::CommandResult, model::prelude::*};
use serenity::{prelude::*, utils::parse_mention};

#[command]
#[checks(Godmod)]
#[only_in(guilds)]
#[aliases("takemod")]
/// Removes a user's mod role.
pub async fn take_mod(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let mention = args.rest();

	let guild = msg
		.guild(&ctx.cache)
		.await
		.expect("Accessing the guild broke.");

	// Get the id of the user mentioned
	let id = parse_mention(mention).expect("Mention was invalid");

	// Remove the mod role from the user
	let mut member = guild.member(&ctx.http, id).await?;
	member.remove_role(&ctx.http, MOD_ROLE_ID).await?;

	msg.react(&ctx.http, '👍').await?;

	Ok(())
}
