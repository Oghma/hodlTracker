//! Commands and command handler functions.
use reqwest::Client;
use serde_json::json;
use teloxide::{prelude::*, utils::command::BotCommands, Bot};

// TODO: Avoid to hardcode italian strings and commands. Generalize
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commandi supportati:")]
pub enum UserCommands {
    #[command(description = "Mostra questo messaggio.")]
    Help,
    #[command(description = "Inserisce una nuova entrata.", parse_with = "split")]
    Entrata {
        flow_name: String,
        amount: f64,
        category: String,
        note: String,
    },
    #[command(description = "Inserisce una nuova entrata.", parse_with = "split")]
    Uscita {
        flow_name: String,
        amount: f64,
        category: String,
        note: String,
    },
    #[command(description = "Lista di tutte le entrate e uscite")]
    Sommario,
}

pub async fn handle_user_commands(
    bot: Bot,
    cfg: super::ConfigParameters,
    msg: Message,
    cmd: UserCommands,
) -> ResponseResult<()> {
    match cmd {
        UserCommands::Help => {
            bot.send_message(msg.chat.id, UserCommands::descriptions().to_string())
                .await?
        }
        UserCommands::Entrata {
            flow_name,
            amount,
            category,
            note,
        } => {
            send_entry(
                &cfg.client,
                &cfg.server,
                &flow_name,
                amount,
                &category,
                &note,
            )
            .await?;
            bot.send_message(msg.chat.id, format!("Ops.")).await?
        }
        UserCommands::Uscita {
            flow_name,
            amount,
            category,
            note,
        } => {
            send_entry(
                &cfg.client,
                &cfg.server,
                &flow_name,
                -amount,
                &category,
                &note,
            )
            .await?;
            bot.send_message(msg.chat.id, format!("Ops.")).await?
        }
        UserCommands::Sommario => bot.send_message(msg.chat.id, "TODO".to_string()).await?,
    };

    Ok(())
}

async fn send_entry(
    client: &Client,
    url: &str,
    flow_name: &String,
    amount: f64,
    category: &String,
    note: &String,
) -> ResponseResult<()> {
    let kwargs = json!({"flow_name": flow_name, "amount":amount, "category":category, "note":note});
    client
        .post(format!("{}{}", url, "/entry"))
        .json(&kwargs)
        .send()
        .await?;
    Ok(())
}
