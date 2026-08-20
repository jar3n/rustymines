use ratatui::backend::CrosstermBackend;

use crossterm::event;

use color_eyre::{Result};
use strum::{EnumIter, IntoStaticStr};

use crate::ui::{
    render,
    Views
};

use crate::event::{
    handle_game_events,
    handle_start_events
};

use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, EnumIter, IntoStaticStr)]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
    Extreme
}

impl FromStr for DifficultyLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<DifficultyLevel, ()> {
        match s {
            "Easy" | "easy" => Ok(DifficultyLevel::Easy),
            "Medium" | "medium" => Ok(DifficultyLevel::Medium),
            "Hard" | "hard" => Ok(DifficultyLevel::Hard),
            "Extreme" | "extreme" => Ok(DifficultyLevel::Extreme),
            _ => Err(()),
        }
    }
}

impl ToString for DifficultyLevel {
    fn to_string(&self) -> String {
        match self {
            DifficultyLevel::Easy => String::from("Easy"),
            DifficultyLevel::Medium => String::from("Medium"),
            DifficultyLevel::Hard => String::from("Hard"),
            DifficultyLevel::Extreme => String::from("Extreme"),
        }
    }
}



pub struct App {
    should_quit: bool,
    difficulty: DifficultyLevel,
    state: Views,
}


impl App {

    pub fn new() -> Self {
        Self {
            should_quit: false,
            difficulty: DifficultyLevel::Easy,
            state: Views::Start,
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
        let event = event::read()?;

        if event.is_key_press() {

            let key_event = event.as_key_event();

            match key_event {
                None => {},
                Some(key_event) => {
                    match &self.state {
                        Views::Start => handle_start_events(self, key_event),
                        Views::InGame => handle_game_events(self, key_event)
                    }
                }
            };
            
        }
        Ok(())
    }

    pub fn difficulty(&self) -> &DifficultyLevel{
        &self.difficulty
    }

    pub fn state(&self) -> &Views {
        &self.state
    }

    pub fn set_difficulty(&mut self, difficulty: &DifficultyLevel) -> Result<()> {
        self.difficulty = *difficulty;
        Ok(())
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn enter_game(&mut self) {
        self.state = Views::InGame;
    }

}