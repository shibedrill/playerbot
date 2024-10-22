use crate::{funcs, types::ServerResponse};
use poise::serenity_prelude as serenity;
use reqwest::{Client, Request};
use serenity::*;
use url::Url;

#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct ServerSummary {
    online: bool,
    players: String,
}

pub struct Data {
    controller: SCPSL,
}

#[derive(Clone)]
pub struct SCPSL {
    url: Url,
    token: String,
}

impl SCPSL {
    pub fn new(url: Url, token: String) -> Self {
        Self { url, token }
    }
    async fn get_status(&self) -> Result<ServerResponse, anyhow::Error> {
        let http_client = Client::new();
        let request = Request::new(reqwest::Method::GET, self.url.clone());
        let response = http_client.execute(request).await?;
        let data: ServerSummary = serde_json::from_str(&response.text().await?)?;

        let playercount: Result<Vec<u32>, _> =
            data.players.split('/').map(|x| x.parse::<u32>()).collect();

        let playercount_unwrapped = playercount?;

        Ok(ServerResponse::new(
            data.online,
            playercount_unwrapped.get(0).map(|u| u.clone()),
            playercount_unwrapped.get(1).map(|u| u.clone()),
        ))
    }

    pub async fn run(&self) {
        let controller = self.clone();
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
            data_about_bot: _data,
        } => loop {
            let status = data
                .controller
                .get_status()
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
