use teloxide::prelude::*;
use teloxide::types::{ChatId, Message};
use warp::Filter;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
struct IncomingHttpMessage {
    message: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");
    let chat_id_str = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not set");
    let chat_id: i64 = chat_id_str.parse().expect("Invalid chat ID format");

    let bot = Bot::from_env().auto_send();

    // Setup HTTP server
    let route = warp::post()
        .and(warp::path("send_message"))
        .and(warp::body::json())
        .map(move |msg: IncomingHttpMessage| {
            let bot = bot.clone();
            tokio::spawn(async move {
                let chat_id = ChatId(chat_id);
                if let Err(e) = bot.send_message(chat_id, msg.message).send().await {
                    eprintln!("Error sending message: {:?}", e);
                }
            });
            warp::reply::json(&"Message received")
        });

    warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}

