#![allow(unused_variables)]
use sqlx_helper::{Create, Get, Delete};

#[derive(Debug, Get, Create, Delete)]
#[table(name = "guild")]
struct Guilds {
    #[get(pk)]
    #[create(ignore)]
    pub id: i64,
    pub channel_id: i64,
}

fn main() {
    let guild = Guilds {
        id: 4,
        channel_id: 8,
    };
    println!("{:?}", guild);
}
