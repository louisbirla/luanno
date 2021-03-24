mod gum;
pub use gum::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(gum)]
struct Player;
