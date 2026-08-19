
use std::rc::Rc;

use crate::app::{
    App,
    DifficultyLevel
};

use ratatui::{
    layout::{
        self, 
        Constraint, 
        Direction, 
        Layout, 
        Rect
    }, 
    style::{
        Color::Green, Style, Stylize
    }, 
    widgets::{
        Block, 
        BorderType, 
        Borders, 
        Paragraph
    }
};




pub enum Views {
    Start,
    InGame,
}

// app render ui entry point
pub fn render(app:&mut App, frame: &mut ratatui::Frame) {
        match app.state() {
            Views::Start => render_start(frame, app),
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

fn level_select_text(level: DifficultyLevel, selected: DifficultyLevel) -> Paragraph<'static> {
    let text_style = match selected == level {
        true => Style::new().bg(Green).black(),
        false => Style::new().green()
    };


    Paragraph::new(
        level.to_string()
    )
    .style(
        text_style
    )
    .centered()
}

// render start menu
pub fn render_start(frame: &mut ratatui::Frame, app: &App) {
    let base = base_layout(frame);
 
    let title_layout = Layout::default()
                                    .direction(Direction::Vertical)
                                    .constraints(vec![
                                        Constraint::Fill(1),
                                        Constraint::Fill(1),
                                        Constraint::Fill(1),
                                        Constraint::Fill(1),
                                        Constraint::Fill(1),
                                    ])
                                    .split(base[0]);

    let top_blank_space = Block::new()
                                        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                                        .border_style(
                                            Style::new()
                                            .green()
                                        )
                                        .border_type(
                                            BorderType::Rounded
                                        );

    frame.render_widget(&top_blank_space, title_layout[0]);

    let title_text = Paragraph::new(
                                            "Welcome to Rusty Mines"
                                            .bold()
                                            .green()
                                        )
                                        .centered()
                                        .block(
                                            Block::new()
                                                .borders(Borders::LEFT | Borders::RIGHT)
                                                .border_style(
                                                    Style::new()
                                                    .green()
                                                    
                                                )
                                                .border_type(
                                                    BorderType::Rounded
                                                )
                                        );
    

    frame.render_widget(title_text, title_layout[1]);

    let instructions_text = Paragraph::new(
                                                            "Select a difficulty using the Left and Right Arrow Keys"
                                                            .green()
                                                            .dim()
                                                        )
                                                        .centered()
                                                        .block(
                                                            Block::new()
                                                            .borders(Borders::LEFT | Borders::RIGHT)
                                                            .green()
                                                        );

    frame.render_widget(instructions_text, title_layout[2]);

    let start_game_instructions = Paragraph::new(
                                                                    "Press Enter and Start Sweeping!"
                                                                    .green()
                                                                    .dim()                                                            
                                                                )
                                                                .centered()
                                                                .block(
                                                                    Block::new()
                                                                    .borders(Borders::LEFT | Borders::RIGHT)
                                                                    .green()
                                                                );
    
    frame.render_widget(start_game_instructions, title_layout[3]);

    let bottom_blank_space = Block::new()
                                        .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                                        .border_style(
                                            Style::new()
                                            .green()
                                        )
                                        .border_type(
                                            BorderType::Rounded
                                        );

    frame.render_widget(&bottom_blank_space, title_layout[title_layout.len()-1]);


    // for temporary implementation
    // put in the selection as static code
    // later condense and make it
    // work with changing selections
    // also clean up
    // the layouts 
    // because there are a lot of empty fills
    // in there

    let selection_vertical_layout = Layout::default()
                                             .constraints(vec![
                                                Constraint::Fill(1),
                                                Constraint::Fill(1),
                                                Constraint::Fill(1),
                                                Constraint::Fill(1),
                                                Constraint::Fill(1),
                                                
                                             ])
                                             .split(base[1]);
    
    let selection_horizontal_layout = Layout::default()
                                                    .direction(Direction::Horizontal)
                                                    .constraints(vec![
                                                        Constraint::Fill(3),
                                                        Constraint::Fill(1),
                                                        Constraint::Fill(3),
                                                        Constraint::Fill(1),
                                                        Constraint::Fill(3),
                                                        Constraint::Fill(1),
                                                        Constraint::Fill(3),
                                                    ])
                                                    .split(selection_vertical_layout[2]);

    frame.render_widget(&top_blank_space, selection_vertical_layout[0]);

    frame.render_widget(&bottom_blank_space, selection_vertical_layout[selection_vertical_layout.len()-1]);

    
    
    frame.render_widget(level_select_text(DifficultyLevel::Easy, app.difficulty()), selection_horizontal_layout[1]);
    frame.render_widget(level_select_text(DifficultyLevel::Medium, app.difficulty()), selection_horizontal_layout[3]);
    frame.render_widget(level_select_text(DifficultyLevel::Hard, app.difficulty()), selection_horizontal_layout[5]);


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

