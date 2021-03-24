mod ping;
pub use ping::*;
mod help;
pub use help::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping)]
struct General;
