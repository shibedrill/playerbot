mod minecraft;
mod scpsl;
mod types;
mod funcs;
use tokio::join;
use dotenvy::{self, dotenv};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    join!(scpsl::run(), minecraft::run());
}
