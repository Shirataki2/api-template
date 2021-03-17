use self::{invite::*, ping::*, down::*};
use serenity::framework::standard::macros::group;

pub mod invite;
pub mod ping;
pub mod down;

#[group]
#[commands(invite, ping, down)]
pub(crate) struct Meta;
