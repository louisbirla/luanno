use super::consts::GODLOG_CHANNEL_ID;
use super::helpers::{db::data_db, name::user_name};
use super::models::history::status::GameStatus;
use log::*;
use serenity::{
	async_trait,
	model::prelude::{Activity, Ready},
};
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
		let data = ctx.data.read().await;
		let db = data_db(&data);
		let status = GameStatus::new(db).await.unwrap();
		let mut message = format!("Round {}...", status.round.id);
		if let Some(player) = status.player {
			let user = player.user(&ctx.http).await.unwrap();
			let name = user_name(&ctx.http, user).await;
			message = format!("{}'s turn ({}/{})", name, status.turn.id, status.turn_count);
		}
		let activity = Activity::playing(&message);
		ctx.shard.set_activity(Some(activity));
	}
}
