use log::*;
use serenity::prelude::*;
use serenity::{async_trait, model::gateway::Ready};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, _ready: Ready) {
		info!("Bot is ready.");
	}
}
