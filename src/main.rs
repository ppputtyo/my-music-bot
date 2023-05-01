mod commands;
mod util;

use commands::{join::*, leave::*, nurupo::*, pause::*, play::*, resume::*, skip::*};
use serenity::client::Context;
use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{standard::macros::group, StandardFramework},
    model::gateway::Ready,
    prelude::GatewayIntents,
};
use songbird::SerenityInit;
use util::get_token;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Bot起動時の処理
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

// 有効なコマンド
#[group]
#[commands(nurupo, join, leave, play, skip, pause, resume)]

struct General;

#[tokio::main]
async fn main() {
    // ログ出力するように設定
    tracing_subscriber::fmt::init();

    // トークンが記述されたconfigファイルを取得
    let token = get_token("config.json").expect("token not found");

    // フレームワークの作成
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    // 特権とされていないintentとメッセージに関するintent
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    // Botのクライアントを作成
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    // クライアントを実行
    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    // Ctrl+Cを検知した場合
    tokio::signal::ctrl_c().await.expect("");
    println!("Received Ctrl-C, shutting down.");
}
