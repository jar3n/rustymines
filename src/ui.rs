
use crate::app::App;


pub enum Views {
    Start,
    InGame,
    Fail,
    Succeed
}


pub fn render(app:&mut App, frame: &mut ratatui::Frame) {
        let greeting = format!("Hello, {}", app.difficulty());
        frame.render_widget(greeting, frame.area());
}