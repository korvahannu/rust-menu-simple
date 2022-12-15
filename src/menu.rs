extern crate ncurses;

use ncurses::*;

struct MenuEntry {
    option: &'static str,
    action: Option<fn() -> ()>
}

pub struct Menu {
    menu_options: Vec<&'static str>,
    menu_actions: Vec<Option<fn() -> ()>>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            menu_options: Vec::new(),
            menu_actions: Vec::new(),
        }
    }
    pub fn add_option(&mut self, option: &'static str, action: Option<fn() -> ()>) {
        self.menu_options.push(option);
        self.menu_actions.push(action);
    }
    pub fn add_empty_line(&mut self) {
        self.menu_options.push("");
        self.menu_actions.push(None);
    }
    pub fn add_text(&mut self, text: &'static str) {
        self.menu_options.push(text);
        self.menu_actions.push(None);
    }
    pub fn remove_at(&mut self, index: usize) {
        self.menu_options.remove(index);
        self.menu_actions.remove(index);
    }
    pub fn remove_option(&mut self, target: &str) {
        let mut i: usize = 0;
        while i < self.menu_options.len() {
            if self.menu_options[i] == target {
                self.menu_options.remove(i);
                self.menu_actions.remove(i);
                break;
            }
            i += 1;
        }
    }
    pub fn remove_options(&mut self, target: &str) {
        let mut i: usize = 0;
        while i < self.menu_options.len() {
            if self.menu_options[i] == target {
                self.menu_options.remove(i);
                self.menu_actions.remove(i);
                i -= 1;
            } else {
                i += 1;
            }
        }
    }
    pub fn start(&self) {
        initscr();
        start_color();
        use_default_colors();
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        init_pair(2, -1, -1);

        let mut selected: usize = 0;
        let mut input: i32 = -1;

        while selected < self.menu_options.len() {
            if Option::is_none(&self.menu_actions[selected]) {
                selected += 1;
            } else {
                break;
            }
        }

        while input != 10 && input != 32 {
            let mut i = 0;
            while i < self.menu_options.len() {
                if Option::is_none(&self.menu_actions[i]) {
                    addstr(format!("{}\n", self.menu_options[i]).as_str());
                } else if selected == i {
                    attron(COLOR_PAIR(1));
                    addstr(format!("{}\n", self.menu_options[i]).as_str());
                    attron(COLOR_PAIR(2));
                } else {
                    addstr(format!("{}\n", self.menu_options[i]).as_str());
                }
                i += 1;
            }

            input = getch();

            if input == 106 || input == 66 {
                if selected < self.menu_actions.len() - 1 {
                    let mut s: usize = selected + 1;
                    while s < self.menu_options.len() {
                        if Option::is_some(&self.menu_actions[s]) {
                            selected = s;
                            break;
                        }
                        s += 1;
                    }
                }
            } else if (input == 107 || input == 65) && selected != 0 {
                let mut s: usize = selected - 1;
                while s != 0 {
                    if Option::is_some(&self.menu_actions[s]) {
                        selected = s;
                        break;
                    }
                    s -= 1;
                }
            }
            clear();
        }
        endwin();
        self.menu_actions[selected].unwrap()();
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}
