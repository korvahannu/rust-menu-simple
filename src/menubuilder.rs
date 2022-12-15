use crate::menu::Menu;
use crate::menuentry::MenuEntry;

pub struct MenuBuilder {
    entries: Vec<MenuEntry>,
}

impl MenuBuilder {
    pub fn new() -> MenuBuilder {
        MenuBuilder {
            entries: Vec::new(),
        }
    }
    pub fn add_option(mut self, option: &'static str, action: Option<fn() -> ()>) -> MenuBuilder {
        self.entries.push(MenuEntry { option, action });
        self
    }
    pub fn add_empty_line(mut self) -> MenuBuilder {
        self.entries.push(MenuEntry {
            option: "",
            action: None,
        });
        self
    }
    pub fn add_text(mut self, text: &'static str) -> MenuBuilder {
        self.entries.push(MenuEntry {
            option: text,
            action: None,
        });
        self
    }
    pub fn build(self) -> Menu {
        Menu::new(self.entries)
    }
}

impl Default for MenuBuilder {
    fn default() -> Self {
        Self::new()
    }
}
