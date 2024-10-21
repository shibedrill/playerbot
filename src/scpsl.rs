
use crate::{funcs, types::ServerResponse};
use poise::serenity_prelude as serenity;
use reqwest::{Client, Request};
use serenity::*;

#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct ServerSummary {
    online: bool,
    players: String,
}

pub struct Data {}

async fn get_status() -> Result<ServerResponse, anyhow::Error> {
    let http_client = Client::new();
    let request = Request::new(
        reqwest::Method::GET,
        url::Url::try_from("https://api.scplist.kr/api/servers/81460")?,
    );
    let response = http_client.execute(request).await?;
    let data: ServerSummary = serde_json::from_str(&response.text().await?)?;

    let playercount: Result<Vec<u32>, _> =
        data.players.split('/').map(|x| x.parse::<u32>()).collect();

    let playercount_unwrapped = playercount?;

    Ok(ServerResponse::new(
        data.online,
        playercount_unwrapped[0],
        playercount_unwrapped[1],
    ))
}

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready {
            data_about_bot: _data,
        } => loop {
            let status = get_status()
                .await
                .inspect_err(|e| error!("Failed to get status: {}", e))
                .unwrap();
            info!("Got status: {}", status.to_string());

            funcs::set_presence(ctx, status);
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        },
        _ => Ok(()),
    }
}

pub async fn run() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            initialize_owners: true,
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
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
    info!("Built framework successfully.");

    let mut discord_client = ClientBuilder::new(
        std::env::var("SCPSL_BOT_TOKEN").inspect_err(|e| {error!("Failed to get token: {}", e)}).unwrap(),
        serenity::GatewayIntents::non_privileged(),
    )
    .framework(framework)
    .activity(ActivityData::custom("Waiting on initial status..."))
    .await
    .inspect_err(|e| error!("Failed to start client: {}", e))
    .unwrap();
    info!("Built client successfully.");

    let _ = discord_client.start().await;
}
