use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn rjtt(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "東京国際空港(羽田)").await?;

    Ok(())
}