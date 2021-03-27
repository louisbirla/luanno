mod now;
pub use now::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(now)]
struct Game;
