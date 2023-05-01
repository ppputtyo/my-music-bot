use crate::util::check_msg;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[only_in(guilds)]
pub(crate) async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    // サーバ情報の取得
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    // メッセージ送信者が参加中のボイスチャンネルを取得
    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    // 接続するボイスチャンネルがなければreturn
    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(
                msg.reply(ctx, "ボイスチャンネル入ってからコマンド送ってね")
                    .await,
            );
            return Ok(());
        }
    };

    // クライアントマネージャの取得
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    // ボイスチャンネルに接続
    let _handler = manager.join(guild_id, connect_to).await;

    check_msg(
        msg.channel_id
            .say(&ctx.http, "ボイスチャンネルに接続しました！")
            .await,
    );

    Ok(())
}
