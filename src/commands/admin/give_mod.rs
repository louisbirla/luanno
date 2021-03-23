use crate::consts::MOD_ROLE_ID;

use super::*;
use serenity::framework::standard::{macros::command, Args};
use serenity::{framework::standard::CommandResult, model::prelude::*};
use serenity::{prelude::*, utils::parse_mention};

#[command]
#[checks(Godmod)]
#[only_in(guilds)]
#[description = "Gives a user the mod role."]
#[aliases("givemod")]
pub async fn give_mod(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let mention = args.rest();

	let guild = msg.guild(&ctx.cache).await.unwrap();

	let id = parse_mention(mention).unwrap();
	let mut member = guild.member(&ctx.http, id).await.unwrap();
	member.add_role(&ctx.http, MOD_ROLE_ID).await.unwrap();

	msg.react(&ctx.http, '👍').await.unwrap();

	Ok(())
}
