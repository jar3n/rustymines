
use crate::app::App;


pub enum Views {
    Start,
    InGame,
}

// app render ui entry point
pub fn render(app:&mut App, frame: &mut ratatui::Frame) {
        
        let menu_widget = match app.state() {
            Views::Start => render_start(),
            Views::InGame => render_game(),
        };

        frame.render_widget(menu_widget, frame.area());
}

// render start menu
pub fn render_start() -> &'static str {
    "Start Menu!!!!"
}

// render in game menu
pub fn render_game() -> &'static str{
    "Game"
}

// might have fail and succeed 
// but makes more sense to have a pop up widget 
// when the win or loss conditions are met

