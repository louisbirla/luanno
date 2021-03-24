mod status;
pub use status::*;
mod manifest;
pub use manifest::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(status, manifest)]
struct Character;
