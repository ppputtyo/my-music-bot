# my-music-bot
[serenity](https://github.com/serenity-rs/serenity)と[songbird](https://github.com/serenity-rs/songbird)を利用したRust製音楽再生botです。

以下のコマンドを実行できます。

|コマンド|機能|
|-|-|
|~nurupo|「ガッ」のAAをメンション付きで返信|
|~join|送信者が参加中のボイスチャンネルに接続|
|~leave|参加中のボイスチャンネルから切断|
|~play {YouTubeのURL}|送信者が参加中のボイスチャンネルでYouTubeを再生 (再生中ならキューに追加)|
|~skip|キューを進めて次の曲を再生|
|~pause|再生中の曲を一時停止|
|~resume|一時停止解除|

詳細は[こちらのQittaの記事](https://qiita.com/ppputtyo/items/bf95c9ccdba3b6042031)で紹介しています。
