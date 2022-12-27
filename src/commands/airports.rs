use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::Args;
use crate::commands::airport_vec::AIRPORTS;

#[command]
async fn airport(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let icao = args.single::<String>()?;
    let airport = AIRPORTS.iter().find(|a| a.icao == icao);
    if let Some(a) = airport
    {
        msg.channel_id.say(&ctx.http, &format!("{}は{}です", icao, a.name)).await?;
    }
    else
    {
        msg.channel_id.say(&ctx.http, "そのICAOコードは登録されていません").await?;
    }
    Ok(())
}

#[command]
async fn code(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single::<String>()?;
    let airport = AIRPORTS.iter().find(|a| a.name == name);
    if let Some(a) = airport {
        msg.channel_id.say(&ctx.http, &format!("{}の空港コードは{}です", name, a.icao)).await?;
    } else {
        msg.channel_id.say(&ctx.http, "その空港名は登録されていません").await?;
    }
    Ok(())
}