/*
Module for thew widget that
displays the minefield

this is simply where the
minefield gets a widget

separate from the minefield 
because its ui stuff

*/

use ratatui::{
    layout::{
        Layout,
        Rect,
        Constraint,
    },
    widgets::{
        Widget,
        Paragraph,
        Block,
    },
    buffer::Buffer,
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
            Paragraph::new(format!("Area {:02}", i + 1))
                .block(Block::bordered())
                .render(cell, buf);
        }

    }
}




