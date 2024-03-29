use itertools::Itertools;
use anyhow::anyhow;

mod structs;
mod utils;
mod args;

fn main() -> Result<(), anyhow::Error> {
    use terminal_menu::{menu, label, button, run, mut_menu};
    
    let args = args::Args::new();

    let steam_profile_id:u64 = match args.profile_id.parse() {
        Ok(value) => value,
        Err(_error) => {
            return Err(anyhow!("Please provide valid Steam profile ID."))
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
        let proton_response = utils::check_proton_db(&game.app_id);

        utils::output(&proton_response, &game.app_id, &game.name);
    }

    Ok(())
}
