use events::Handler;
use log::*;
use serenity::prelude::*;
use serenity::{
	client::bridge::gateway::ShardManager, framework::standard::StandardFramework, http::Http,
};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;

mod commands;
use commands::*;

pub mod consts;
mod events;

// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
	type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
	type Value = HashMap<String, u64>;
}

#[tokio::main]
async fn main() {
	pretty_env_logger::init();

	// Get the discord bot token
	let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

	// Create an HTTP cache & context for the bot
	let http = Http::new_with_token(&token);

	// Fetch the bot's id
	let bot_id = match http.get_current_application_info().await {
		Ok(_) => match http.get_current_user().await {
			Ok(bot_id) => bot_id.id,
			Err(why) => panic!("Could not access the bot id: {:?}", why),
		},
		Err(why) => panic!("Could not access application info: {:?}", why),
	};

	// Configure the bot
	let framework = StandardFramework::new()
		.configure(|c| {
			c.with_whitespace(true)
				.on_mention(Some(bot_id))
				.prefix("!")
				.delimiters(vec![" "])
		})
		.help(&HELP_COMMAND)
		.group(&GENERAL_GROUP)
		.group(&MODCOMMANDS_GROUP);

	// Make the client
	let mut client = Client::builder(&token)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Err creating client");

	{
		let mut data = client.data.write().await;
		data.insert::<CommandCounter>(HashMap::default());
		data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
	}

	if let Err(why) = client.start().await {
		error!("Client error: {:?}", why);
	}
}
