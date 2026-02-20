use poise::serenity_prelude as serenity;
use dotenvy::dotenv;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data
pub struct Data {}

/// Show bot info
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

/// Say hello
#[poise::command(slash_command)]
async fn hello(
    ctx: Context<'_>,
    #[description = "Your name"] name: Option<String>,
) -> Result<(), Error> {
    let response = match name {
        Some(n) => format!("Hello, {}! 👋", n),
        None => "Hello! 👋".to_string(),
    };
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN environment variable");
    
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), hello()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
