use serenity::framework::standard::{
	macros::{check, command},
	Args, CommandOptions, CommandResult, Reason,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

// Checks if you are me. You may be! (probably are not.)
#[check]
#[name = "Godmod"]
pub async fn godmod_check(
	_: &Context,
	msg: &Message,
	_: &mut Args,
	_: &CommandOptions,
) -> Result<(), Reason> {
	if !is_owner(msg.author.id) {
		return Err(Reason::User("Your are not the Godmod.".to_string()));
	}

	Ok(())
}

#[command]
#[description = "Check if you are the Godmod"]
#[aliases("amigodmod")]
pub async fn am_i_godmod(ctx: &Context, msg: &Message) -> CommandResult {
	if is_owner(msg.author.id) {
		msg.reply(&ctx.http, "You are the Godmod.").await?;
	} else {
		msg.reply(&ctx.http, "You are **not** the Godmod.").await?;
	}

	Ok(())
}

pub fn is_owner(user_id: UserId) -> bool {
	user_id == 408649315095937045
}
