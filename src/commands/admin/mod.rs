mod godmod_check;
pub use godmod_check::*;
mod mod_check;
pub use mod_check::*;
mod give_mod;
pub use give_mod::*;
mod take_mod;
pub use take_mod::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(am_i_godmod, am_i_mod, give_mod, take_mod)]
struct ModCommands;
