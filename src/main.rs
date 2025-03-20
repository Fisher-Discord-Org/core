use config::Config;
use serde::Deserialize;
use serenity::all::{ClientBuilder, GuildId, HttpBuilder};
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::{GatewayIntents, Ready};
use std::env;
use tonic::async_trait;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("DEV_GUILD_ID")
                .expect("Expected DEV_GUILD_ID in env")
                .parse()
                .expect("DEV_GUILD_ID must be an integer"),
        );

        println!("Guild ID: {:?}", guild_id);
    }
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Settings {
    debug: bool,
    token: String,
}

fn load_config() -> Result<Settings, config::ConfigError> {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();
    }

    let settings = Config::builder()
        .add_source(config::File::with_name("config").required(false))
        .add_source(config::Environment::with_prefix("DISCORD"))
        .set_default(
            "debug",
            match cfg!(debug_assertions) {
                true => true,
                false => false,
            },
        )?
        .build()?;

    settings.try_deserialize()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let settings = load_config().expect("Failed to load settings");

    println!("{:?}", settings);
    let http = HttpBuilder::new(settings.token)
        // .proxy("http://127.0.0.1:55555")
        .build();

    let user = http
        .get_current_user()
        .await
        .expect("Failed to validate token");

    println!("Logged in as: {}", user.name);

    let mut client = ClientBuilder::new_with_http(http, GatewayIntents::default())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    println!("Client created");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
