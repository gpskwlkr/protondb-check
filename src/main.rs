use args::Args;
use itertools::Itertools;
use anyhow::{anyhow, Result, Context};

mod structs;
mod utils;
mod args;

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
    
    use terminal_menu::{menu, label, button, run, mut_menu};

    let steam_profile_id:u64 = match args.profile_id.unwrap().parse() {
        Ok(value) => value,
        Err(_error) => {
            return Err(anyhow!("Please provide valid Steam profile ID.")).with_context(|| "handle_profile_id")
        }
    };

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
    let app_id = args.app_id.unwrap();
    let proton_response = utils::check_proton_db(&app_id)?;
    utils::output(&proton_response, &app_id, None);

    Ok(())
}
