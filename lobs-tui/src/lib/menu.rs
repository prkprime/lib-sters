#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Hottest,
    Newest,
    Saved,
    Preference,
    Quit,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Hottest => 0,
            MenuItem::Newest => 1,
            MenuItem::Saved => 2,
            MenuItem::Preference => 3,
            MenuItem::Quit => 5,
        }
    }
}
