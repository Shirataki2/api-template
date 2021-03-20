pub mod join;
pub mod leave;
use serenity::framework::standard::macros::group;

use self::{join::*, leave::*};

#[group]
#[commands(join, leave)]
pub(crate) struct Tts;
