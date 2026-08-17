use ratatui::backend::CrosstermBackend;
use crossterm::event;


use color_eyre::Result;

use crate::ui::render;


pub struct App {
    should_quit: bool,
    name: String,
}


impl App {

    pub fn new(name: String) -> Self {
        Self {
            should_quit: false,
            name: name
        }

    }

    pub fn run(&mut self, terminal: &mut ratatui::Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()>{
        while !self.should_quit {
            terminal.draw(
                |frame| render(self, frame)
            )?;
                self.handle_events()?;
        }
        Ok(())
    }



    fn handle_events(&mut self) -> Result<()>{
        if event::read()?.is_key_press() {
            self.should_quit = true;
        }
        Ok(())
    }

    pub fn name(&mut self) -> String{
        self.name.clone()
    }
}