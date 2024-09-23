use crate::{Context, Error};

/// 音楽の再生を再開
#[poise::command(slash_command, prefix_command, guild_only)]
pub(crate) async fn resume(ctx: Context<'_>) -> Result<(), Error> {
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

        // 一時停止解除
        queue.resume().expect("Resume failed");

        ctx.say("一時停止解除").await?;
    } else {
        ctx.say("ボイスチャンネルに入ってないよ").await?;
    }

    Ok(())
}
