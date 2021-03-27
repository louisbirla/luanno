use serenity::{http::CacheHttp, model::prelude::User};

use crate::consts::GUILD_ID;

pub async fn user_name(cache_http: &impl CacheHttp, user: User) -> String {
	user.nick_in(cache_http, GUILD_ID)
		.await
		.unwrap_or(user.name)
}
