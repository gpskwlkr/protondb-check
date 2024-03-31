use anyhow::{anyhow, Context, Result};
use args::Args;
use itertools::Itertools;

mod args;
mod structs;
mod utils;

fn main() -> Result<()> {
    let args = Args::new()?;

    args.validate_args()?;

    if args.app_id.is_some() {
        handle_app_id(args)?;
    } else {
        handle_profile_id(args)?;
    }

    Ok(())
}

fn handle_profile_id(args: Args) -> Result<()> {
    use terminal_menu::{button, label, menu, mut_menu, run};

    let steam_profile_id: u64 = args
        .profile_id
        .ok_or_else(|| anyhow!("Please provide a valid Steam profile ID."))
        .and_then(|id| {
            id.parse()
                .map_err(|_| anyhow!("Please provide a valid Steam profile ID."))
        })?;

    let games_list = utils::get_games_list(steam_profile_id)?;

    let mut menu_items = vec![
        label("----------------------"),
        label("ProtonDB Helper"),
        label("You can use w/s, j/k or arrow keys to move around"),
        label("enter to select"),
        label("'q' or esc to exit"),
        label("-----------------------"),
    ];

    for name in games_list.keys().sorted() {
        menu_items.push(button(name));
    }

    let menu = menu(menu_items);
    run(&menu);
    {
        let mm = mut_menu(&menu);
        let game = games_list.get(mm.selected_item_name()).unwrap();
        let proton_response = utils::check_proton_db(&game.app_id)?;

        utils::output(&proton_response, &game.app_id, Some(&game.name));
    }

    Ok(())
}

fn handle_app_id(args: Args) -> Result<()> {
    let app_id: u32 = args
        .app_id
        .ok_or_else(|| anyhow!("App ID not provided or invalid"))
        .with_context(|| "handle_app_id")?;

    let proton_response = utils::check_proton_db(&app_id)?;
    utils::output(&proton_response, &app_id, None);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{args::Args, handle_app_id};

    #[test]
    fn test_handle_app_id_valid() {
        let args = Args {
            app_id: Some(870780),
            profile_id: None,
        };

        let result = handle_app_id(args);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_app_id_invalid() {
        let args = Args {
            app_id: None,
            profile_id: None,
        };

        let result = handle_app_id(args);

        assert!(result.is_err());
        assert_eq!("handle_app_id", result.unwrap_err().to_string());
    }
}
