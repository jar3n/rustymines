use ratatui::backend::CrosstermBackend;
use crossterm::event;


use color_eyre::Result;
use strum::{EnumIter, IntoStaticStr};


use crate::ui::{
    render,
    Views
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

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
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
            difficulty: DifficultyLevel::Hard,
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
        if event::read()?.is_key_press() {
            self.should_quit = true;
        }
        Ok(())
    }

    pub fn difficulty(&self) -> DifficultyLevel{
        self.difficulty
    }

    pub fn state(&self) -> &Views {
        &self.state
    }

    // pub fn set_difficulty(&mut self, difficulty: str) -> Result<()> {
    //     match  {

    //     }
    // }
}