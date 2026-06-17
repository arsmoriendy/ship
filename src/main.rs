#[macro_use]
mod utils;
mod config;
mod image;
mod prelude;
mod project;
mod registry;
mod relation;
mod tui;

use prelude::*;

fn main() -> Result<()> {
    let mut app = tui::App::new()?;
    let mut terminal = ratatui::init();
    app.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
