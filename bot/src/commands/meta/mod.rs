use self::{down::*, invite::*, ping::*};
use serenity::framework::standard::macros::group;

pub mod down;
pub mod invite;
pub mod ping;

#[group]
#[commands(invite, ping, down)]
pub(crate) struct Meta;
