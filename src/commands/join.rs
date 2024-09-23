use crate::{Context, Error};

/// 発言者が参加中のボイスチャンネルに参加
#[poise::command(slash_command, prefix_command, guild_only)]
pub(crate) async fn join(ctx: Context<'_>) -> Result<(), Error> {
    join_voice_channel(ctx).await?;

    Ok(())
}

pub(crate) async fn join_voice_channel(ctx: Context<'_>) -> Result<(), Error> {
    // サーバ情報の取得
    // ctx.guild().unwrap()で返ってくるCacheRefがSendではないため、awaitを跨がないようにスコープを制限する
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();

        let guild_id = guild.id;
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild_id, channel_id)
    };

    // 接続するボイスチャンネルがなければreturn
    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.reply("ボイスチャンネル入ってからコマンド送ってね")
                .await?;
            return Ok(());
        }
    };

    // クライアントマネージャの取得
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    // ボイスチャンネルに接続
    manager.join(guild_id, connect_to).await?;

    ctx.say("ボイスチャンネルに接続しました！").await?;

    Ok(())
}
