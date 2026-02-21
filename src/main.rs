use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
use tracing::{info, error};

mod service;
mod commands;

use commands::{price, cashflow, balancesheet, incomestatement};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data
pub struct Data {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    dotenv().ok();
    
    info!("Starting Discord bot...");
    
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN environment variable");
    
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![price(), cashflow(), balancesheet(), incomestatement()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                info!("Registering slash commands...");
                
                // For development: register to specific guilds for instant updates
                let guild_ids = std::env::var("GUILD_IDS")
                    .ok()
                    .map(|ids| {
                        ids.split(',')
                            .filter_map(|id| id.trim().parse::<u64>().ok())
                            .map(serenity::GuildId::new)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                
                if !guild_ids.is_empty() {
                    info!("Registering commands to {} guild(s) (instant)", guild_ids.len());
                    for guild_id in guild_ids {
                        poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                    }
                } else {
                    info!("Registering commands globally (takes up to 1 hour)");
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }
                
                info!("Bot is ready!");
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    match client {
        Ok(mut client) => {
            if let Err(why) = client.start().await {
                error!("Client error: {:?}", why);
            }
        }
        Err(why) => {
            error!("Failed to create client: {:?}", why);
        }
    }
}
