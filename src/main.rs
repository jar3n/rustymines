pub mod app;

pub mod ui;

pub mod minefield;

pub mod event;

pub mod boardui;

use std::io;

use app::App;




use color_eyre::Result;
use crossterm::{
    execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},

};
use ratatui::{Terminal, backend::CrosstermBackend};



fn main() -> Result<()> {
    // start
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    // mid 
    let mut app = App::new();

    app.run(&mut terminal)?;


    // end

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    terminal.show_cursor()?;


    Ok(())
}
