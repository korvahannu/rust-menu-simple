pub struct MenuEntry {
    pub option: &'static str,
    pub action: Option<fn() -> ()>,
}
