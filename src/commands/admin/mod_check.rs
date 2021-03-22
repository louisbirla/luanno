use serenity::prelude::*;
use serenity::Error;
use serenity::{framework::standard::CommandResult, model::prelude::*};
use serenity::{
	framework::standard::{
		macros::{check, command},
		Args, CommandOptions, Reason,
	},
	http::CacheHttp,
};

use crate::consts::{GUILD_ID, MOD_ROLE_ID};

// Checks if you are me. You may be! (probably are not.)
#[check]
#[name = "Mod"]
pub async fn mod_check(
	ctx: &Context,
	msg: &Message,
	_: &mut Args,
	_: &CommandOptions,
) -> Result<(), Reason> {
	match is_mod(&ctx.http, &msg.author).await {
		Ok(is) if is => Ok(()),
		Ok(_) => Err(Reason::User("Your are not a mod.".to_string())),
		Err(_) => Err(Reason::Unknown),
	}
}

#[command]
#[description = "Check if you are a mod"]
#[aliases("amimod")]
pub async fn am_i_mod(ctx: &Context, msg: &Message) -> CommandResult {
	if is_mod(&ctx.http, &msg.author).await? {
		msg.reply(&ctx.http, "You are a mod.").await?;
	} else {
		msg.reply(&ctx.http, "You are **not** a mod.").await?;
	}

	Ok(())
}

pub async fn is_mod(cache_http: impl CacheHttp, user: &User) -> Result<bool, Error> {
	user.has_role(cache_http, GUILD_ID, MOD_ROLE_ID).await
}
