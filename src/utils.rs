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
const STEAM_XML_URL: &str = "https://steamcommunity.com/profiles/";

pub fn get_games_list(steam_id: u64) -> Result<HashMap<String, Game>> {
    let url = format!("{}{}/games?tab=all&xml=1", STEAM_XML_URL, steam_id);
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

fn calculate_percent(score: f32) -> u32 {
    if score >= 1.00 {
        score as u32
    } else {
        (score * 100.0) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_proton_db_valid() {
        let app_id: u32 = 870780;

        let result = check_proton_db(&app_id);

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_proton_db_invalid() {
        let app_id: u32 = 1;

        let result = check_proton_db(&app_id);

        assert!(result.is_err());
        assert_eq!("check_proton_db", result.unwrap_err().to_string());
    }

    #[test]
    fn test_get_games_list_valid() {
        let steam_profile_id: u64 = 76561198354374976;

        let result = get_games_list(steam_profile_id);

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_games_list_invalid() {
        let steam_profile_id: u64 = 1;

        let result = get_games_list(steam_profile_id);

        assert!(result.is_err());
        assert_eq!("get_games_list", result.unwrap_err().to_string());
    }

    #[test]
    fn test_calculate_percent() {
        let result = calculate_percent(0.7132);

        assert_eq!(result, 71);
    }
}
