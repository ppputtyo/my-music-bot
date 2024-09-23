mod commands;
mod util;

use poise::serenity_prelude as serenity;

use commands::{
    join::join, leave::leave, nurupo::nurupo, pause::pause, play::play, resume::resume, skip::skip,
};

use reqwest::Client as HttpClient;
use songbird::SerenityInit;
use util::get_token;

type Error = Box<dyn std::error::Error + Send + Sync>;

struct Data {
    http: HttpClient,
}

type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let commands = vec![
        register(),
        nurupo(),
        join(),
        leave(),
        pause(),
        play(),
        resume(),
        skip(),
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
            on_error: |error| {
                Box::pin(async move {
                    println!("what the hell");
                    match error {
                        poise::FrameworkError::ArgumentParse { error, .. } => {
                            if let Some(error) = error.downcast_ref::<serenity::RoleParseError>() {
                                println!("Found a RoleParseError: {:?}", error);
                            } else {
                                println!("Not a RoleParseError :(");
                            }
                        }
                        other => poise::builtins::on_error(other).await.unwrap(),
                    }
                })
            },
            ..Default::default()
        })
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    http: HttpClient::new(),
                })
            })
        })
        .build();

    let token = get_token("config.json").expect("token not found");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;

    client.unwrap().start().await.unwrap()
}
