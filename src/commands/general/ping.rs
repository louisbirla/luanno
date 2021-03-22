use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, "gnip!").await?;

	Ok(())
}
