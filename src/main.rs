use rustmenu::menubuilder::*;

fn main() {
    main_menu();
}

fn main_menu() {
    MenuBuilder::new()
        .add_text("▲ The Dark Forest ▲")
        .add_empty_line()
        .add_text("Use the arrow keys and enter or j and k with space to change and select options.")
        .add_empty_line()
        .add_option("1. Start Game", Some(start_game))
        .add_option("3. Quit Game", Some(null))
        .build()
        .start();
}

fn start_game() {
    MenuBuilder::new()
        .add_text("You walk into a dark forest, what do you do?")
        .add_empty_line()
        .add_option("1. You turn back and leave", Some(c1o1))
        .add_option("2. You make haste and run through the dark forest", Some(c1o2))
        .build()
        .start();
}

fn c1o1() {
    MenuBuilder::new()
        .add_text("You leave the dark forest never to return again.")
        .add_empty_line()
        .add_option("1. Return to menu", Some(main_menu))
        .add_option("2. Quit Game", Some(null))
        .build()
        .start();
}

fn c1o2() {
    MenuBuilder::new()
        .add_text("As you make haste through the dark forest, a magical elf stops you and asks for a smoke.")
        .add_empty_line()
        .add_option("1. Smoking is bad, so you decide punch the magical elf", Some(c2o1))
        .add_option("2. You give the magical elf a smoke and light one up for yourself as well", Some(c2o2))
        .add_option("3. You apologize and tell the magical elf you do not have any cigarettes with you", Some(c2o3))
        .build()
        .start();
}

fn c2o1() {
    MenuBuilder::new()
        .add_text("The magical elf gets angered and instakills you.")
        .add_text("You lost the game!")
        .add_empty_line()
        .add_option("1. Return to menu", Some(main_menu))
        .add_option("2. Quit Game", Some(null))
        .build()
        .start();
}

fn c2o2() {
    MenuBuilder::new()
        .add_text("The magical elf says 'Sike!' and lights you on fire. You die.")
        .add_text("You lost the game!")
        .add_empty_line()
        .add_option("1. Return to menu", Some(main_menu))
        .add_option("2. Quit Game", Some(null))
        .build()
        .start();
}

fn c2o3() {
    MenuBuilder::new()
        .add_text("The magical elf is impressed with your politiness. You suddenly teleport back to your home with a bag of gold.")
        .add_text("You won the game!")
        .add_empty_line()
        .add_option("1. Return to menu", Some(main_menu))
        .add_option("2. Quit Game", Some(null))
        .build()
        .start();
}

fn null() {}
