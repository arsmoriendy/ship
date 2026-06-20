#[macro_use]
mod utils;
mod config;
mod image;
mod prelude;
mod project;
mod relation;
mod store;
mod tui;

use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = tui::App::new()?;
    let mut terminal = ratatui::init();
    app.run(&mut terminal).await?;
    ratatui::restore();
    Ok(())
}
