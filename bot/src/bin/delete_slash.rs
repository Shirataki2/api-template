use std::env;

use anyhow::Result;
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
    let application_id = http.get_current_application_info().await.map(|op| op.id)?.0;
    if opt.global {
        let commands = http.get_global_application_commands(application_id).await?;
        for command in commands.iter() {
            http.delete_global_application_command(application_id, command.id.0)
                .await?;
            eprintln!("Delete Command: {:#?}", command);
        }
    } else {
        let commands = http
            .get_guild_application_commands(application_id, opt.id)
            .await?;
        for command in commands.iter() {
            http.delete_guild_application_command(application_id, opt.id, command.id.0)
                .await?;
            eprintln!("Delete Command: {:#?}", command);
        }
    }

    Ok(())
}
