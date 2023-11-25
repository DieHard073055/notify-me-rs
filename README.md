# Telegram Bot Server

This repository contains the source code for a Telegram bot server. The server listens for HTTP POST requests and forwards received messages to a specified Telegram chat using a Telegram bot.

## Prerequisites

- Rust programming language.
- Cargo (Rust's package manager).
- A Telegram bot token and chat ID.

## Setting Up

To run this server, you will need to set the following environment variables:

1. `TELOXIDE_TOKEN`: Your Telegram bot token, which you can get from [BotFather](https://t.me/botfather) on Telegram.

    ```bash
    export TELOXIDE_TOKEN=<Your_Telegram_Bot_Token>
    ```

2. `TELEGRAM_CHAT_ID`: The chat ID where messages will be forwarded.

    ```bash
    export TELEGRAM_CHAT_ID=<Your_Telegram_Chat_ID>
    ```

Replace `<Your_Telegram_Bot_Token>` and `<Your_Telegram_Chat_ID>` with your actual Telegram bot token and chat ID.

## Running the Server

To run the server, use the following command:

```bash
cargo run
```

The server will start and listen for incoming HTTP POST requests.

## Testing the Server

To test if the server is functioning correctly, you can send a test message using `curl`:

```bash
curl -X POST http://localhost:3030/send_message \
     -H "Content-Type: application/json" \
     -d '{"message": "Hello, this is a test message from curl"}'
```

This command sends a POST request to the server with a JSON payload containing a test message. The server should then forward this message to the specified Telegram chat.

