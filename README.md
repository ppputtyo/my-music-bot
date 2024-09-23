# my-music-bot

[serenity](https://github.com/serenity-rs/serenity)、[songbird](https://github.com/serenity-rs/songbird)、[poise](https://github.com/serenity-rs/poise)を利用した Rust 製音楽再生 bot です。

## 実行方法

1. `git clone https://github.com/ppputtyo/my-music-bot/`
1. `git switch dev_poise`
1. `config.json`の作成
   ```json
   {
     "token": "DISCORD_TOKEN"
   }
   ```
1. `cargo run -r`

## 実行可能コマンド

以下のコマンドを実行できます。

| コマンド               | 機能                                                                       |
| ---------------------- | -------------------------------------------------------------------------- |
| ~nurupo                | 「ガッ」の AA をメンション付きで返信                                       |
| ~join                  | 送信者が参加中のボイスチャンネルに接続                                     |
| ~leave                 | 参加中のボイスチャンネルから切断                                           |
| ~play {YouTube の URL} | 送信者が参加中のボイスチャンネルで YouTube を再生 (再生中ならキューに追加) |
| ~skip                  | キューを進めて次の曲を再生                                                 |
| ~pause                 | 再生中の曲を一時停止                                                       |
| ~resume                | 一時停止解除                                                               |

詳細は[こちらの Qitta の記事](https://qiita.com/ppputtyo/items/3b0ea4bca40c7d0ab563)で紹介しています。
