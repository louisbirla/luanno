use crate::consts::MOD_ROLE_ID;

use super::*;
use serenity::framework::standard::{macros::command, Args};
use serenity::{framework::standard::CommandResult, model::prelude::*};
use serenity::{prelude::*, utils::parse_mention};

#[command]
#[checks(Godmod)]
#[only_in(guilds)]
#[description = "Removes a user's mod role."]
#[aliases("takemod")]
pub async fn take_mod(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let mention = args.rest();

	let guild = msg.guild(&ctx.cache).await.unwrap();

	let id = parse_mention(mention).unwrap();
	let mut member = guild.member(&ctx.http, id).await.unwrap();
	member.remove_role(&ctx.http, MOD_ROLE_ID).await.unwrap();

	msg.react(&ctx.http, '👍').await.unwrap();

	Ok(())
}
