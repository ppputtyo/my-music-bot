use crate::{Context, Error};

/// 再生中の音楽を中断する
#[poise::command(slash_command, prefix_command, guild_only)]
pub(crate) async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    // クライアントマネージャの取得
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.pause().expect("Pause failed");

        ctx.say("一時停止中…").await?;
    } else {
        ctx.say("ボイスチャンネルに入ってないよ").await?;
    }

    Ok(())
}
