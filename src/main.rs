use rustmenu::menu::*;

fn main() {
    let mut menu = Menu::new();

    menu.add_text("What would you like to do?");
    menu.add_empty_line();
    menu.add_option("1. New Game", Some(menu_start_game));
    menu.add_option("2. Options", Some(menu_options));
    menu.add_option("3. End Game", Some(menu_end_game));

    menu.start();
}

fn menu_start_game() {
    println!("Start Game");
}

fn menu_options() {
    println!("Options");
}

fn menu_end_game() {
    println!("End Game");
}
