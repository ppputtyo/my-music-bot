use crate::{Context, Error};

use serenity::prelude::Mentionable;

/// リプライ付きで「がっ」する
#[poise::command(prefix_command, slash_command)]
pub(crate) async fn nurupo(ctx: Context<'_>) -> Result<(), Error> {
    let res = format!(
        r"{}
```
　　 （　・∀・）　　　|　|　ｶﾞｯ
　　と　　　　）　 　 |　|
　　　 Ｙ　/ノ　　　 人
　　　　 /　）　 　 < 　>__Λ∩
　　 ＿/し'　／／. Ｖ｀Д´）/
　　（＿フ彡　　　　　 　　/
```",
        ctx.author().mention()
    );

    ctx.say(res).await?;

    Ok(())
}
