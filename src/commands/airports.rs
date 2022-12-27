use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::Args;
use crate::commands::airport_vec::AIRPORTS;

#[command]
async fn airport(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let icao = args.single::<String>()?;
    if let Some(name) = AIRPORTS.get(&icao) {
        msg.channel_id.say(&ctx.http, &format!("{}は {} 空港(飛行場,基地,駐屯地...)です。", icao, name)).await?;
    } else {
        msg.channel_id.say(&ctx.http, "そのICAOコードは登録されていません（空港のコードはすべて大文字で入力してください）").await?;
    }
    Ok(())
}

#[command]
async fn code(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single::<String>()?;
    //let icao = AIRPORTS.iter().find(|(_, v)| v == &name);
    let icao = AIRPORTS.iter().find(|(_, v)| v.as_str() == &name);
    if let Some((icao, _)) = icao {
        msg.channel_id.say(&ctx.http, &format!("{}空港(飛行場、基地...)の空港コードは **{}** です", name, icao)).await?;
    } else {
        msg.channel_id.say(&ctx.http, "その空港名は登録されていません（「空港」等は入れずに入力してください）").await?;
    }
    Ok(())
}