

use std::rc::Rc;

use crate::app::{
    App,
    DifficultyLevel
};

use ratatui::{
    layout::{
        self, Constraint, Direction, Layout, Rect
    }, style::{
        Color::{Green, LightBlue, LightMagenta, Red, White, Yellow}, Style, Stylize
    }, text::Line, widgets::{
        Block, 
        BorderType, 
        Borders, 
        List, 
        ListState, 
        Paragraph,
        canvas::{
            Canvas,
            Rectangle,
        }

    }
};
use strum::IntoEnumIterator;



pub enum Views {
    Start,
    InGame,
}

// app render ui entry point
pub fn render(app:&mut App, frame: &mut ratatui::Frame) {
        match app.state() {
            Views::Start => render_start(frame, app),
            Views::InGame => render_game(frame, app),
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
                                                            "Select a difficulty using 'W' and 'S'"
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

    let mut items: Vec<Line> = vec![];
    
    for level in DifficultyLevel::iter() {
        let level_str: &'static str = level.into();
        items.push(Line::from(level_str).centered());
    };

    let items_clone = items.clone();

    let level_selection = List::new(items)
                                            .block(Block::new()
                                                    .borders(Borders::ALL)
                                                        .green()
                                                        .border_type(BorderType::Rounded)
                                                    )
                                            .highlight_style( 
                                                Style::new().bg(Green).black()
                                            )
                                            .style(
                                                Green,
                                            );
                                            
    let selected_difficulty_str: &'static str = app.difficulty().into();
    let selected_difficulty_index = items_clone.iter().position(|r| *r.to_string() == *selected_difficulty_str);                                        

    let mut state = ListState::default();
    state.select(selected_difficulty_index);
    

    frame.render_stateful_widget(level_selection, base[1], &mut state);



}


fn tile(num_columns: f64, num_rows: f64, column: f64, row: f64, revealed: bool, is_selected: bool, is_flagged: bool, tiletype: i8) ->Rectangle {
    // todo 
    // add size adjustment 
    // and translate row column coordinates
    // to the center of the tile

    Rectangle { 
        x: column, 
        y: row, 
        width: 1.0/num_columns, 
        height: 1.0/num_rows, 
        color: if is_selected {
            Green
        } else if is_flagged {
            LightMagenta
        } else if revealed {
            match tiletype {
                -1 => Red,
                0 => LightBlue,
                _ => Yellow,
            }
        } else {
            White
        }
    }
}

// render in game menu
pub fn render_game(frame: &mut ratatui::Frame, app: &App) {
    let base = base_layout(frame);

    // todo
    // figure out why the curser
    // title is not changing 
    // when i press the buttons
    // might be because 
    // it does not have state??

    let board_layout = Layout::default()
                                        .direction(Direction::Horizontal)
                                        .constraints(
                                            vec!{
                                                Constraint::Fill(1),
                                                Constraint::Fill(1),
                                                Constraint::Fill(1)
                                            }
                                        )
                                        .split(base[0]);


    let rows = app.field().rows();
    let columns = app.field().columns();

    // render the board in the top
    // temporary just render one tile
    let board = Canvas::default()
                                        .x_bounds([0.0, rows as f64])
                                        .y_bounds([0.0, columns as f64])
                                        .marker(ratatui::symbols::Marker::Quadrant)
                                        .paint(|ctx| {
                                            for row in 0..rows {
                                                for column in 0..columns {
                                                    let tile_state = app.field().is_bomb(row, column);
                                                    let is_selected = column == app.selected_tile()[0] && row == app.selected_tile()[1];
                                                    let is_flagged = app.field().is_flagged(row, column);
                                                    let is_revealed = app.field().is_revealed(row, column);

                                                    let num_neighbors = match tile_state {
                                                        true => -1,
                                                        false => app.field().check_neighbors(row,column)
                                                    };

                                                    let tile = tile(
                                                        columns as f64,
                                                        rows as f64,
                                                        column as f64,
                                                        row as f64,
                                                        is_revealed, 
                                                        is_selected && !(app.has_lost() || app.has_won()),
                                                        is_flagged,
                                                        num_neighbors
                                                    );

                                                    ctx.draw(&tile);

                                                    if is_revealed && !app.field().is_bomb(row, column){
                                                        let tile_type_str = format!("{}", num_neighbors);

                                                        ctx.print(column as f64 + tile.width/2.0, row as f64, tile_type_str);
                                                    }
                                                    
                                                }
                                            }
                                            
                                            
                                        })
                                        .block(Block::new()
                                                .borders(Borders::TOP | Borders::BOTTOM)
                                                .border_type(BorderType::Rounded)
                                                .green()
                                                );

    frame.render_widget(board, board_layout[1]);

    frame.render_widget(Block::new()
                                        .green()
                                        .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                                        .border_type(BorderType::Rounded)
                                        .title(
                                            if app.has_lost() {
                                                format!("Level: {:?}, Size: {} by {}, Number of Bombs: {}, You Lost", 
                                                        app.difficulty(), 
                                                        columns, 
                                                        rows,
                                                        app.field().num_bombs()
                                                        )
                                            } else if app.has_won() {
                                                    format!("Level: {:?}, Size: {} by {}, Number of Bombs: {}, You WON!!!!!", 
                                                        app.difficulty(), 
                                                        columns, 
                                                        rows,
                                                        app.field().num_bombs()
                                                        )
                                            } else {
                                                format!("Level: {:?}, Size: {} by {}, Number of Bombs: {}", 
                                                        app.difficulty(), 
                                                        columns, 
                                                        rows,
                                                        app.field().num_bombs()
                                                        )
                                            }
                                            
                                            ),
                         board_layout[0]);

    frame.render_widget(Block::new()
                                        .green()
                                        .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
                                        .border_type(BorderType::Rounded),
                         board_layout[2]);

    // render the instructions in the bottom

    let instructions_layout = Layout::default()
                                                .constraints(vec![
                                                    Constraint::Fill(1),
                                                    Constraint::Fill(1),
                                                    Constraint::Fill(1),
                                                    Constraint::Fill(1),
                                                    Constraint::Fill(1)
                                                ])
                                                .split(base[1]);

    let instructions_pt1 = Paragraph::new("Navigate the board using WASD")
                                                            .block(Block::new()
                                                                          .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
                                                                          .border_type(BorderType::Rounded)
                                                                          .green()      
                                                        ).centered();
    let instructions_pt2 = Paragraph::new("Flag a tile with F")
                                                            .block(Block::new()
                                                                          .borders(Borders::LEFT | Borders::RIGHT)
                                                                          .border_type(BorderType::Rounded)
                                                                          .green()
                                                                                
                                                        ).centered();
    
    let instructions_pt3 = Paragraph::new("Reveal a Tile with Enter")
                                                            .block(Block::new()
                                                                          .borders(Borders::LEFT | Borders::RIGHT)
                                                                          .border_type(BorderType::Rounded)
                                                                          .green()      
                                                        ).centered();
    
    let instructions_pt4 = Paragraph::new("Return to the Start with B")
                                                            .block(Block::new()
                                                                          .borders(Borders::LEFT | Borders::RIGHT)
                                                                          .border_type(BorderType::Rounded)
                                                                          .green()      
                                                        ).centered();
    
    let instructions_pt5 = Paragraph::new("Quit the game with Q")
                                                            .block(Block::new()
                                                                          .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                                                                          .border_type(BorderType::Rounded)
                                                                          .green()      
                                                        ).centered();


    frame.render_widget(instructions_pt1, instructions_layout[0]);
    frame.render_widget(instructions_pt2, instructions_layout[1]);
    frame.render_widget(instructions_pt3, instructions_layout[2]);
    frame.render_widget(instructions_pt4, instructions_layout[3]);
    frame.render_widget(instructions_pt5, instructions_layout[4]);



}

// might have fail and succeed 
// but makes more sense to have a pop up widget 
// when the win or loss conditions are met

