use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use tracing::{error, info};

mod commands;
mod service;

use commands::{balancesheet, cashflow, incomestatement, price};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data
pub struct Data {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    info!("Starting Discord bot...");

    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN environment variable");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![price(), cashflow(), balancesheet(), incomestatement()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                info!("Registering slash commands...");

                // Check if we're in development mode
                let is_dev = std::env::var("ENVIRONMENT")
                    .unwrap_or_else(|_| "production".to_string())
                    .to_lowercase()
                    == "development";

                if is_dev {
                    // Development: register to specific guilds for instant updates
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
                        info!(
                            "Development mode: Registering commands to {} guild(s) (instant)",
                            guild_ids.len()
                        );
                        for guild_id in guild_ids {
                            poise::builtins::register_in_guild(
                                ctx,
                                &framework.options().commands,
                                guild_id,
                            )
                            .await?;
                        }
                    } else {
                        info!("Development mode: No GUILD_IDS specified, registering globally");
                        poise::builtins::register_globally(ctx, &framework.options().commands)
                            .await?;
                    }
                } else {
                    // Production: always register globally
                    info!("Production mode: Registering commands globally (takes up to 1 hour)");
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
