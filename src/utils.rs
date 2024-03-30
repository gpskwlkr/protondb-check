use anyhow::{anyhow, Context, Result};
use reqwest::blocking::get;
use std::collections::HashMap;

use crate::structs::{Game, GamesList, ProtonAPIResponse};

const KNOWN_NOT_GAMES: &[u32] = &[
    431960, // Wallpaper Engine
    365670, // Blender
    431730, // Aseprite
];

const PROTON_API_URL: &str = "https://www.protondb.com/api/v1/reports/summaries/";

pub fn get_games_list(steam_id: u64) -> Result<HashMap<String, Game>> {
    let url = format!(
        "https://steamcommunity.com/profiles/{}/games?tab=all&xml=1",
        steam_id
    );
    let response = get(url).unwrap();
    let xml_string = response.text().unwrap();
    let games_list: GamesList = match serde_xml_rs::from_str(&xml_string) {
        Ok(value) => value,
        Err(_error) => {
            return Err(anyhow!(
                "Unable to retrieve Steam data. Is Steam profile ID valid?"
            ))
            .with_context(|| "get_games_list")
        }
    };

    let game_map: HashMap<String, Game> = games_list
        .games
        .game
        .into_iter()
        .filter(|game| !KNOWN_NOT_GAMES.contains(&game.app_id) && game.hours_on_record.is_some())
        .map(|game| (game.name.clone(), game))
        .collect();

    Ok(game_map)
}

pub fn check_proton_db(app_id: &u32) -> Result<ProtonAPIResponse> {
    let url = format!("{}{}.json", PROTON_API_URL, app_id);
    let response_text = get(url).unwrap().text().unwrap();
    let api_response = match serde_json::from_str(&response_text) {
        Ok(value) => value,
        Err(_error) => {
            return Err(anyhow!(
                "Unable to retrieve data. Possibly not found in ProtonDB"
            ))
            .with_context(|| "check_proton_db")
        }
    };

    Ok(api_response)
}

pub fn output(response: &ProtonAPIResponse, app_id: &u32, game: Option<&str>) {
    println!("----------------------");
    println!("App ID: {}", app_id);
    if let Some(game_name) = game {
        println!("Game: {}", game_name);
    } else {
        println!("Note: game name couldn't be fetched in this mode.");
    }
    println!("General Tier: {}", response.tier);
    println!("Trending Tier: {}", response.trending_tier);
    println!("Success chance: {}%", calculate_percent(response.score));
    println!("Trending tier difference may involve latest proton updates.");
}

fn calculate_percent(score: f32) -> f32 {
    if score >= 1.00 {
        score
    } else {
        ((score * 100.0) as i32) as f32
    }
}
