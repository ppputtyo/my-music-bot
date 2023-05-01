use crate::{commands::join, util::check_msg};

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use songbird::input::Restartable;

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // 引数からURLを取得
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "URL頂戴").await);
            return Ok(());
        }
    };

    // httpから始まらない場合はエラー
    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "ちゃんとしたURL頂戴").await);
        return Ok(());
    }

    // サーバ情報の取得
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    // クライアントマネージャの取得
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    // ボイスチャンネルに接続していない場合は接続
    if let None = manager.get(guild_id) {
        join::join(ctx, msg, args)
            .await
            .expect("Voice channel connection failed");
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // URLから音楽をダウンロード
        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(
                    msg.channel_id
                        .say(&ctx.http, "FFmpegが見つかりません")
                        .await,
                );

                return Ok(());
            }
        };

        // 再生キューに音楽を追加
        handler.enqueue_source(source.into());

        // 現在のキューの長さを取得
        let queue_len = handler.queue().len();
        if queue_len == 1 {
            check_msg(msg.channel_id.say(&ctx.http, "再生中～～").await);
        } else {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("{}曲後に再生されるよ", queue_len - 1))
                    .await,
            );
        }
    }

    Ok(())
}
