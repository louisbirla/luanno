pub mod ping;
pub use ping::*;
pub mod help;
pub use help::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping)]
struct General;
