use crate::{commands::join, Context, Error};

use songbird::input::YoutubeDl;

/// URLの音楽を再生する
/// 現在音楽を再生中の場合はキューに追加
#[poise::command(slash_command, prefix_command, guild_only)]
pub(crate) async fn play(
    ctx: Context<'_>,
    #[description = "youtube url"] url: String,
) -> Result<(), Error> {
    // httpから始まらない場合はエラー
    if !url.starts_with("http") {
        ctx.say("ちゃんとしたURL頂戴").await?;
        return Ok(());
    }

    // サーバ情報の取得
    let guild_id = ctx.guild_id().unwrap();

    // クライアントマネージャの取得
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    // ボイスチャンネルに接続していない場合は接続
    if manager.get(guild_id).is_none() {
        join::join_voice_channel(ctx)
            .await
            .expect("Voice channel connection failed");
    }

    // スラッシュコマンドは3秒間応答がないとコマンドが失敗したと判断される
    // そのため、時間のかかる動画ダウンロード処理の前に応答し、コマンド失敗を防ぐ
    // https://discord.com/developers/docs/interactions/receiving-and-responding
    ctx.defer().await?;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let http_client = ctx.data().http.clone();
        let src = YoutubeDl::new(http_client, url);

        handler.enqueue_input(src.into()).await;

        ctx.say(format!("キューに追加: position {}", handler.queue().len()))
            .await?;
    } else {
        ctx.say("ボイスチャンネルに入ってないよ").await?;
    }

    Ok(())
}
