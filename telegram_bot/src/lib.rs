//! Library for the telegram bot
use reqwest::Client;
use teloxide::{prelude::*, Bot as TBot};

use crate::handlers::{handle_user_commands, UserCommands};

mod handlers;

#[derive(Clone)]
pub struct ConfigParameters {
    allowed_users: Option<Vec<UserId>>,
    client: Client,
    server_url: String,
}

pub struct Bot {
    token: String,
    allowed_users: Option<Vec<UserId>>,
    server_url: String,
}

impl Bot {
    pub fn new(token: &str, allowed_users: Option<Vec<UserId>>, server_url: &str) -> Self {
        Self {
            token: token.to_string(),
            allowed_users,
            server_url: server_url.to_string(),
        }
    }

    pub fn builder() -> BotBuilder {
        BotBuilder::default()
    }

    pub async fn run(&self) {
        tracing::info!("Starting telegram bot...");

        let bot = TBot::new(&self.token);
        let parameters = ConfigParameters {
            allowed_users: self.allowed_users.clone(),
            client: Client::new(),
            server_url: self.server_url.clone(),
        };

        let handler = Update::filter_message().branch(
            dptree::filter(|cfg: ConfigParameters, msg: Message| {
                msg.from()
                    .map(|user| match cfg.allowed_users {
                        None => true,
                        Some(ids) => ids.contains(&user.id),
                    })
                    .unwrap_or_default()
            })
            .filter_command::<UserCommands>()
            .endpoint(handle_user_commands),
        );

        Dispatcher::builder(bot, handler)
            .dependencies(dptree::deps![parameters])
            .default_handler(|upd| async move {
                tracing::warn!("Unhandled update {:?}", upd);
            })
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occured in the dispatcher",
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}

#[derive(Default, Debug)]
pub struct BotBuilder {
    token: String,
    allowed_users: Option<Vec<UserId>>,
    server_url: String,
}

impl BotBuilder {
    pub fn token(mut self, token: &str) -> BotBuilder {
        self.token = token.to_string();
        self
    }

    pub fn allowed_users(mut self, allowed_users: Vec<UserId>) -> BotBuilder {
        if !allowed_users.is_empty() {
            self.allowed_users = Some(allowed_users);
        }
        self
    }

    pub fn server_url(mut self, server_url: &str) -> BotBuilder {
        self.server_url = server_url.to_string();
        self
    }

    pub fn build(self) -> Bot {
        tracing::info!("Initializing...");
        Bot {
            token: self.token,
            allowed_users: self.allowed_users,
            server_url: self.server_url,
        }
    }
}