mod funcs;
mod minecraft;
mod scpsl;
mod types;
use dotenvy::{self, dotenv};
use minecraft::Minecraft;
use scpsl::Scpsl;
use tokio::join;
use url::{self, Url};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    let gamerzone = Minecraft::new(
        Url::try_from("https://api.mcstatus.io/v2/status/java/gamer.shibedrill.site").unwrap(),
        std::env::var("TOKEN_BOT_MC_GAMER").unwrap(),
        "Gamer SMP".into(),
    );
    let mchprs = Minecraft::new(
        Url::try_from("https://api.mcstatus.io/v2/status/java/mchprs.shibedrill.site").unwrap(),
        std::env::var("TOKEN_BOT_MC_MCHPRS").unwrap(),
        "Project MCRV".into(),
    );
    let scpsl = Scpsl::new(
        Url::try_from("https://api.scplist.kr/api/servers/81460").unwrap(),
        std::env::var("TOKEN_BOT_SCPSL").unwrap(),
        "SCP: SL".into(),
    );
    join!(scpsl.run(), mchprs.run(), gamerzone.run());
}
