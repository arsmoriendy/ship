#[macro_use]
mod utils;
mod image;
mod prelude;
mod project;
mod store;
mod tui;

use clap::Parser;
use prelude::*;

#[derive(Parser)]
#[command(version, about)]
struct Args {}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = Args::parse();
    let mut app = tui::App::new()?;
    let mut terminal = ratatui::init();
    app.run(&mut terminal).await?;
    ratatui::restore();
    Ok(())
}
