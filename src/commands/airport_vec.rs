use once_cell::sync::Lazy;

pub struct Airport {
    pub icao: String,
    pub name: String,
}

pub static AIRPORTS: Lazy<Vec<Airport>> = Lazy::new(|| vec![
    Airport
    {
        icao: "RJFF".to_string(),
        name: "福岡国際空港".to_string(),
    },
    Airport
    {
        icao: "RJTT".to_string(),
        name: "東京国際空港 (成田)".to_string(),
    },
    // 他の空港データもここに追加していく
]);