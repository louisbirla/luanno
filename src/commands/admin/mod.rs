pub mod godmod_check;
pub use godmod_check::*;
pub mod mod_check;
pub use mod_check::*;
pub mod give_mod;
pub use give_mod::*;
pub mod take_mod;
pub use take_mod::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(am_i_godmod, am_i_mod, give_mod, take_mod)]
struct ModCommands;
