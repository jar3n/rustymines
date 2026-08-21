
use crossterm::event::{
    KeyEvent,
    KeyCode
};

use crate::app::{App, DifficultyLevel, Direction};

fn move_level_select_up(current_selection: &DifficultyLevel) -> DifficultyLevel {
    // might change this to use the iterator instead

    match current_selection {
        DifficultyLevel::Easy => DifficultyLevel::Extreme,
        DifficultyLevel::Extreme => DifficultyLevel::Hard,
        DifficultyLevel::Hard => DifficultyLevel::Medium,
        DifficultyLevel::Medium => DifficultyLevel::Easy,
        }
}

fn move_level_select_down(current_selection: &DifficultyLevel) -> DifficultyLevel {
    // might change this to use the iterator instead

    match current_selection {
        DifficultyLevel::Easy => DifficultyLevel::Medium,
        DifficultyLevel::Medium => DifficultyLevel::Hard,
        DifficultyLevel::Hard => DifficultyLevel::Extreme,
        DifficultyLevel::Extreme => DifficultyLevel::Easy,
        }
}

pub fn handle_start_events(app: &mut App, key_event: KeyEvent) {
    
    match key_event.code {
        KeyCode::Char('W') | KeyCode::Char('w') => app.set_difficulty(&move_level_select_up(app.difficulty())).unwrap(),
        KeyCode::Char('S') | KeyCode::Char('s') => app.set_difficulty(&move_level_select_down(app.difficulty())).unwrap(),
        KeyCode::Char('Q') | KeyCode::Char('q') => app.quit(),
        KeyCode::Enter => app.enter_game(),

        _ => {},
    }
}

pub fn handle_game_events(app: &mut App, key_event: KeyEvent) {

    // todo
    // add flagging tiles
    // and revealing tiles
    match key_event.code {
       KeyCode::Char('W') | KeyCode::Char('w') => app.move_selected_tile(Direction::Up),
       KeyCode::Char('S') | KeyCode::Char('s') => app.move_selected_tile(Direction::Down),
       KeyCode::Char('A') | KeyCode::Char('a') => app.move_selected_tile(Direction::Left),
       KeyCode::Char('D') | KeyCode::Char('d') => app.move_selected_tile(Direction::Right),
       KeyCode::Char('Q') | KeyCode::Char('q') => app.quit(),

       _ => {},
    }
}