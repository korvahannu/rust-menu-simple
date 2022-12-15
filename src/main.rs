use rustmenu::menubuilder::*;

fn main() {
    let menu = MenuBuilder::new()
        .add_text("What would you like to do?")
        .add_empty_line()
        .add_option("1. New Game", Some(menu_start_game))
        .add_option("2. Options", Some(menu_options))
        .add_option("3. End Game", Some(menu_end_game))
        .build();

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
