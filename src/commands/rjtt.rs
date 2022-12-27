use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::Args;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use std::sync::Arc;

// mod airport_vec;
// use self::airport_vec::AIRPORTS;
use crate::commands::airport_vec::AIRPORTS;

// #[derive(Debug)]
// struct Airport {
//     icao: String,
//     name: String,
// }

// static AIRPORTS: Lazy<Vec<Airport>> = Lazy::new(|| vec![
//     Airport
//     {
//         icao: "RJFF".to_string(),
//         name: "福岡国際空港".to_string(),
//     },
//     Airport
//     {
//         icao: "RJTT".to_string(),
//         name: "東京国際空港 (成田)".to_string(),
//     },
//     // 他の空港データもここに追加していく
// ]);

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