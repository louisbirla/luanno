pub mod status;
pub use status::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(status)]
struct Character;
