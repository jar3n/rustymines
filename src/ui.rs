
use std::rc::Rc;

use crate::app::{
    App,
    DifficultyLevel
};

use ratatui::{
    layout::{
        self, Constraint, Direction::Vertical, Layout, Rect
    }, style::{Color, Style}, symbols, widgets::{
        Block, BorderType, Borders, Paragraph
    }
};




pub enum Views {
    Start,
    InGame,
}

// app render ui entry point
pub fn render(app:&mut App, frame: &mut ratatui::Frame) {
        match app.state() {
            Views::Start => render_start(frame),
            Views::InGame => render_game(frame),
        };
}

fn base_layout(frame: &mut ratatui::Frame) -> Rc<[Rect]>{
    layout::Layout::default()
            .constraints(vec![
                Constraint::Percentage(66),
                Constraint::Percentage(33)
            ])
            .split(frame.area())
}

pub fn widget_borders() -> Block<'static> {
    Block::new()
        .borders(Borders::ALL)
        .border_style(
            Style::new()
            .green()
            
        )
        .border_type(
            BorderType::Rounded
        )
}

// render start menu
pub fn render_start(frame: &mut ratatui::Frame) {
    let base = base_layout(frame);

    let title_layout = Layout::default()
                                    .direction(Vertical)
                                    .constraints(vec![
                                        Constraint::Fill(1),
                                        Constraint::Fill(1),
                                        Constraint::Fill(1),
                                    ])
                                    .split(base[0]);

    let top_text = Paragraph::new("Rusty Mines")
                                        .centered()
                                        .block(widget_borders());

    frame.render_widget(top_text, title_layout[1]);

    let bottom_text = Paragraph::new("start menu options go here")
                                        .centered()
                                        .block(widget_borders());

    frame.render_widget(bottom_text, base[1]);
}

// render in game menu
pub fn render_game(frame: &mut ratatui::Frame) {
    let base = base_layout(frame);

    let text = Paragraph::new("game with paragraph");

    frame.render_widget(text, base[0]);
}

// might have fail and succeed 
// but makes more sense to have a pop up widget 
// when the win or loss conditions are met

