use crate::{Context, Error};

/// キュー内の音楽を1つスキップ
#[poise::command(slash_command, prefix_command, guild_only)]
pub(crate) async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    // サーバ情報の取得
    let guild_id = ctx.guild_id().unwrap();

    // クライアントマネージャの取得
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        // キューの長さを取得
        let queue_len = queue.len();
        if queue_len == 0 {
            ctx.say("スキップする曲がないよ").await?;
            return Ok(());
        }

        queue.skip().expect("Skip Failed");

        ctx.say(format!("スキップ成功: あと{}曲残ってるよ ", queue_len - 1))
            .await?;
    } else {
        ctx.say("ボイスチャンネルに入ってないよ").await?;
    }

    Ok(())
}
