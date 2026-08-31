/*
Module for thew widget that
displays the minefield

this is simply where the
minefield gets a widget

separate from the minefield 
because its ui stuff

*/

use ratatui::{
    buffer::Buffer, layout::{
        Constraint, Layout, Rect,
    }, style::{
        Color::{
            Blue, Green, LightMagenta, Yellow, White, Red
        }, Stylize
    }, widgets::{
        Block, Paragraph, Widget,
    },

};

use crate::minefield::MineField;



impl Widget for MineField {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let col_constraints = (0..self.columns()).map(|_| Constraint::Length(9));
        let row_constraints = (0..self.rows()).map(|_| Constraint::Length(3));
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {

            if i == self.selected_square() {
                if self.is_revealed_index(i) {
                    if self.is_selected_bomb() {
                        Paragraph::new("")
                                    .block(Block::bordered())
                                    .bg(Red)
                                    .fg(Red)
                                    .render(cell, buf);
                    } else {
                        let num_bomb_neighbors = self.check_neighbors_index(i);
                        Paragraph::new(format!("{}", num_bomb_neighbors))
                                .block(Block::bordered())
                                .bg(Green)
                                .fg(Green)
                                .render(cell, buf);
                    }
                    
                } else  {
                    Paragraph::new("")
                                .block(Block::bordered())
                                .bg(Green)
                                .fg(Green)
                                .render(cell, buf);
                } 
                
            } else if self.is_revealed_index(i){
                 let num_bomb_neighbors = self.check_neighbors_index(i);
                    Paragraph::new(format!("{}", num_bomb_neighbors))
                                .block(Block::bordered())
                                .bg(if num_bomb_neighbors == 0 {
                                    Blue
                                } else {
                                    Yellow
                                })
                                .fg(if num_bomb_neighbors == 0 {
                                    Blue
                                } else {
                                    Yellow
                                })
                                .render(cell, buf);
            } else if self.is_flagged_index(1) {
                 Paragraph::new("")
                                .block(Block::bordered())
                                .bg(LightMagenta)
                                .fg(LightMagenta)
                                .render(cell, buf);
            } else {
                Paragraph::new("")
                                .block(Block::bordered())
                                .bg(White)
                                .fg(White)
                                .render(cell, buf);
            }
        }
    }
}




