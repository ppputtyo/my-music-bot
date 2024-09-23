use crate::{Context, Error};

/// 参加中のボイスチャンネルから切断
#[poise::command(slash_command, prefix_command, guild_only)]
pub(crate) async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    // サーバ情報の取得
    let guild_id = ctx.guild_id().unwrap();

    // クライアントマネージャの取得
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    // Botがサーバのボイスチャンネルに参加中ならTrue
    if manager.get(guild_id).is_some() {
        // サーバのボイスチャンネルから切断
        if let Err(e) = manager.remove(guild_id).await {
            ctx.say(format!("Failed: {:?}", e)).await?;
        }
        ctx.say("ボイスチャンネルから切断したよ").await?;
    } else {
        ctx.say("ボイスチャンネルに入ってないよ").await?;
    }

    Ok(())
}
