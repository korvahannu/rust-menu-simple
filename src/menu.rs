use ncurses::*;

use crate::menuentry::MenuEntry;

pub struct Menu {
    entries: Vec<MenuEntry>,
}

impl Menu {
    pub fn new(entries: Vec<MenuEntry>) -> Menu {
        Menu { entries }
    }
    pub fn start(&self) {
        initscr();
        start_color();
        use_default_colors();
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        init_pair(2, -1, -1);

        let mut selected: usize = 0;
        let mut input: i32 = -1;

        while selected < self.entries.len() {
            if Option::is_none(&self.entries[selected].action) {
                selected += 1;
            } else {
                break;
            }
        }

        while input != 10 && input != 32 {
            let mut i = 0;
            while i < self.entries.len() {
                if Option::is_none(&self.entries[i].action) {
                    addstr(format!("{}\n", self.entries[i].option).as_str());
                } else if selected == i {
                    attron(COLOR_PAIR(1));
                    addstr(format!("{}\n", self.entries[i].option).as_str());
                    attron(COLOR_PAIR(2));
                } else {
                    addstr(format!("{}\n", self.entries[i].option).as_str());
                }
                i += 1;
            }

            input = getch();

            if input == 106 || input == 66 {
                if selected < self.entries.len() - 1 {
                    let mut s: usize = selected + 1;
                    while s < self.entries.len() {
                        if Option::is_some(&self.entries[s].action) {
                            selected = s;
                            break;
                        }
                        s += 1;
                    }
                }
            } else if (input == 107 || input == 65) && selected != 0 {
                let mut s: usize = selected - 1;
                while s != 0 {
                    if Option::is_some(&self.entries[s].action) {
                        selected = s;
                        break;
                    }
                    s -= 1;
                }
            }
            clear();
        }
        endwin();
        self.entries[selected].action.unwrap()();
    }
}
