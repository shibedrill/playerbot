use poise::serenity_prelude as serenity;
use reqwest::{Client, Request};
use serenity::*;
use url::Url;

use crate::{funcs, types::ServerResponse};

#[derive(serde::Deserialize)]
struct ServerSummary {
    online: bool,
    players: Option<Players>,
}

#[derive(serde::Deserialize)]
struct Players {
    online: i32,
    max: i32,
}

pub struct Data {
    controller: Minecraft,
}

#[derive(Clone)]
pub struct Minecraft {
    url: Url,
    token: String,
}

impl Minecraft {
    pub fn new(url: Url, token: String) -> Self {
        Self { url, token }
    }

    pub async fn get_status(&self) -> Result<ServerResponse, anyhow::Error> {
        let http_client = Client::new();
        let request = Request::new(reqwest::Method::GET, self.url.clone());
        let response = http_client.execute(request).await?;
        let data: ServerSummary = serde_json::from_str(&response.text().await?)?;
        if let Some(players) = data.players {
            Ok(ServerResponse::new(
                data.online,
                Some(players.online as u32),
                Some(players.max as u32),
            ))
        } else {
            Ok(ServerResponse::new(data.online, None, None))
        }
    }
    pub async fn run(&self) {
        let controller = self.clone();
        let framework = poise::Framework::builder()
            .options(poise::FrameworkOptions {
                event_handler: |ctx, event, framework, data| {
                    Box::pin(event_handler(ctx, event, framework, data))
                },
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data { controller })
                })
            })
            .build();
        info!("Built framework successfully.");

        let mut discord_client = ClientBuilder::new(
            self.token.clone(),
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
}

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready {
            data_about_bot: _bot_data,
        } => loop {
            let status = data
                .controller
                .get_status()
                .await
                .inspect_err(|e| error!("Failed to get status: {}", e))
                .unwrap();
            info!("Got status: {}", status.to_string());

            funcs::set_presence(&ctx, status);
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        },
        _ => Ok(()),
    }
}
