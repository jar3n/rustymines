use ratatui::backend::CrosstermBackend;

use crossterm::event;

use color_eyre::{Result};
use strum::{EnumIter, IntoStaticStr};

use crate::minefield::MineField;
use crate::ui::{
    render,
    Views
};

use crate::event::{
    handle_game_events,
    handle_start_events
};

use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, EnumIter, IntoStaticStr, Debug)]
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

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}


pub struct App {
    should_quit: bool,
    difficulty: DifficultyLevel,
    state: Views,
    field: Option<MineField>,
    selected_tile: (usize, usize) // column, row
}


impl App {

    pub fn new() -> Self {
        Self {
            should_quit: false,
            difficulty: DifficultyLevel::Easy,
            state: Views::Start,
            field: None,
            selected_tile: (0,0)
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
        // set the view and make the 
        // minefield based on the difficulty
        
        self.state = Views::InGame;

        match self.difficulty {
            DifficultyLevel::Easy => self.field = Some(MineField::new(5, 5, 3)),
            DifficultyLevel::Medium => self.field = Some(MineField::new(7, 7, 14)),
            DifficultyLevel::Hard => self.field = Some(MineField::new(9, 9, 27)),
            DifficultyLevel::Extreme => self.field = Some(MineField::new(10, 10, 40)),
        }
    }

    pub fn field(self: &Self) -> Option<MineField> {
        self.field.clone()
    }

    pub fn selected_tile(self: &Self) -> (usize, usize) {
        self.selected_tile.clone()
    }

    pub fn move_selected_tile(self: &mut Self, direction: Direction) {

        match direction {
            Direction::Down => {
                let new_tile = self.selected_tile;

                match new_tile.1.checked_add(1) {
                    None => {},
                    Some(_) => self.selected_tile = new_tile
                }
            },
            Direction::Up => {
                let new_tile = self.selected_tile;

                match new_tile.1.checked_sub(1) {
                    None => {},
                    Some(_) => self.selected_tile = new_tile
                }
            },
            Direction::Left => {
                 let new_tile = self.selected_tile;

                match new_tile.0.checked_sub(1) {
                    None => {},
                    Some(_) => self.selected_tile = new_tile
                }
            },
            Direction::Right => {
                 let new_tile = self.selected_tile;

                match new_tile.0.checked_add(1) {
                    None => {},
                    Some(_) => self.selected_tile = new_tile
                }
            },      
        }
    }

}