use capitoltrades_discord_bot::{
    endpoints::{
        help::{self, help},
        trades::{list_callback, trades},
    },
    markdown::trades,
    Data,
};
use poise::serenity_prelude::GatewayIntents;
use serenity::all::ClientBuilder;
use sqlx::PgPool;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![help(), trades(), list_callback()],

            prefix_options: poise::PrefixFrameworkOptions {
                ..Default::default()
            },

            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;
    println!("ok");
    match client.start_autosharded().await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Error at event loop: {}", e);
            Err(e.into())
        }
    }
}
