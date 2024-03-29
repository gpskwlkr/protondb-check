use std::collections::HashMap;
use reqwest::blocking::get;

use crate::structs::{GamesList, Game, ProtonAPIResponse};

const KNOWN_NOT_GAMES: &[u32] = &[
    431960,  // Wallpaper Engine
    365670,  // Blender 
    431730,  // Aseprite
];

const PROTON_API_URL: &str = "https://www.protondb.com/api/v1/reports/summaries/";

pub fn get_games_list(steam_id: u64) -> HashMap<String, Game> {
    let url = format!("https://steamcommunity.com/profiles/{}/games?tab=all&xml=1", steam_id);
    let response = get(url).unwrap();
    let xml_string = response.text().unwrap();
    let games_list: GamesList = serde_xml_rs::from_str(&xml_string).unwrap();
    
    let game_map: HashMap<String, Game> = games_list.games.game.into_iter()
        .filter(|game| !KNOWN_NOT_GAMES.contains(&game.app_id) && game.hours_on_record.is_some())
        .map(|game| (game.name.clone(), game))
        .collect();

    game_map 
}

pub fn check_proton_db(app_id: &u32) -> ProtonAPIResponse {
    let url = format!("{}{}.json", PROTON_API_URL, app_id);
    let response_text = get(url).unwrap().text().unwrap();
    let api_response: ProtonAPIResponse = serde_json::from_str(&response_text).expect("Failed to deserialize JSON");

    api_response
}

pub fn output(response: &ProtonAPIResponse, app_id: &u32, game: &str) {
    println!("----------------------");
    println!("App ID: {}", app_id);
    println!("Game: {}", game);
    println!("General Tier: {}", response.tier);
    println!("Trending Tier: {}", response.trending_tier);
    println!("Trending tier difference may involve latest proton updates.");
}
