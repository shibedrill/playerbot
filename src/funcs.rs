use poise::serenity_prelude::{ActivityData, OnlineStatus};

use crate::types::ServerResponse;

pub fn set_presence(ctx: &poise::serenity_prelude::Context, status: ServerResponse) {
    ctx.set_presence(
        Some(ActivityData::custom(match status.online() {
            true => {
                format!(
                    "{}/{} players online",
                    status.players().unwrap(),
                    status.max().unwrap()
                )
            }
            false => "Server offline!".to_string(),
        })),
        match status.online() {
            true => match status.is_full() {
                true => OnlineStatus::Idle,
                false => OnlineStatus::Online,
            },
            false => OnlineStatus::DoNotDisturb,
        },
    );
}
