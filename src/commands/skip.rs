use crate::util::check_msg;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    // サーバ情報の取得
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    // クライアントマネージャの取得
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        // キューの長さを取得
        let queue_len = queue.len();
        if queue_len == 0 {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("スキップする曲がないよ"))
                    .await,
            );
            return Ok(());
        }

        queue.skip().expect("Skip Failed");

        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("スキップ成功: あと{}曲残ってるよ ", queue_len - 1),
                )
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "ボイスチャンネルに入ってないよ")
                .await,
        );
    }

    Ok(())
}
