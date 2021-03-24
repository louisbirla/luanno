use events::Handler;
use log::*;
use mongodb::{options::ClientOptions, Client as MongoClient, Database};
use serenity::prelude::*;
use serenity::{framework::standard::StandardFramework, http::Http};
use std::{env, sync::Arc};

mod commands;
use commands::*;

pub mod consts;
mod events;
pub mod helpers;
pub mod models;

struct DatabaseConnection;

impl TypeMapKey for DatabaseConnection {
	type Value = Arc<Database>;
}

#[tokio::main]
async fn main() {
	pretty_env_logger::init();

	// Get the discord bot token
	let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

	// Create an HTTP cache & context for the bot
	let http = Http::new_with_token(&token);

	let client_options =
		ClientOptions::parse(&env::var("MONGO_URL").expect("Expected the MongoDB connection URL"))
			.await
			.expect("Something broke when making client options for MongoDB");
	let client = MongoClient::with_options(client_options)
		.expect("Accessing the MongoDB client didn't end well.");
	let db = client.database(&env::var("DB_NAME").expect("Expected the DB name"));

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
		.bucket("hourly", |b| b.delay(60 * 60))
		.await
		.help(&HELP_COMMAND)
		.group(&GENERAL_GROUP)
		.group(&PLAYER_GROUP)
		.group(&CHARACTER_GROUP)
		.group(&MODCOMMANDS_GROUP);

	// Make the client
	let mut client = Client::builder(&token)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Err creating client");

	{
		let mut data = client.data.write().await;
		data.insert::<DatabaseConnection>(Arc::new(db));
	}

	if let Err(why) = client.start().await {
		error!("Client error: {:?}", why);
	}
}
