mod app;
mod maze;

use app::App;
use color_eyre::Result;
use maze::Maze;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
