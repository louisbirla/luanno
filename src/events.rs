use super::consts::GODLOG_CHANNEL_ID;
use log::*;
use serenity::{async_trait, model::gateway::Ready};
use serenity::{model::id::ChannelId, prelude::*};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, ctx: Context, _ready: Ready) {
		info!("Bot is ready.");
		ChannelId(GODLOG_CHANNEL_ID)
			.say(&ctx.http, "[:robot:] Bot is ready")
			.await
			.expect("Couldn't send ready message to godlog channel");
	}
}
