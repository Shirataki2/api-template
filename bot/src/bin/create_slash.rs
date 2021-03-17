use std::env;

use anyhow::Result;
use bot::schema::SLASH_COMMANDS;
use dotenv::dotenv;
use serenity::http::Http;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short)]
    global: bool,
    #[structopt(short)]
    id: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let opt = Opt::from_args();

    let token = env::var("DISCORD_BOT_TOKEN").expect("Missing DISCORD_BOT_TOKEN");
    let http = Http::new_with_token(&token);
    if opt.global {
        for command in SLASH_COMMANDS.iter() {
            http.create_global_application_command(
                http.get_current_application_info().await.map(|op| op.id)?.0,
                command,
            )
            .await?;
            eprintln!("Add Command: {:#?}", &command);
        }
    } else {
        for command in SLASH_COMMANDS.iter() {
            http.create_guild_application_command(
                http.get_current_application_info().await.map(|op| op.id)?.0,
                opt.id,
                command,
            )
            .await?;
            eprintln!("Add Command: {:#?}", &command);
        }
    }

    Ok(())
}
