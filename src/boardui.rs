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
    }, 
    style::{
        Color::{
            Black, 
            Blue, 
            Green, 
            LightMagenta, 
            Red, 
            White, 
            Yellow
        }, 
        Style,
    },  
    widgets::{
        Block, Paragraph, Widget,
    },

};

use crate::minefield::MineField;

impl MineField {
    fn cell_style(self: &Self, index: usize) -> Style {
        let mut style = Style::new();

        if self.selected_square() == index {
            if self.is_revealed_index(index) {
                if self.is_selected_bomb() {
                    style = style.fg(Red);
                    style = style.bg(Red);
                } else {
                    style = style.fg(Black);
                    style = style.bg(Green);
                }

            } else {
                style = style.fg(Green);
                style = style.bg(Green);
            }

        } else if self.is_revealed_index(index) {
            let num_bomb_neighbors = self.check_neighbors_index(index);

            if num_bomb_neighbors == 0 {
                style = style.fg(Blue);
                style = style.bg(Blue)
            } else {
                style = style.fg(Black);
                style = style.bg(Yellow);
            }

        } else if self.is_flagged_index(index) {
            style = style.fg(LightMagenta);
            style = style.bg(LightMagenta);

        } else {
            style = style.fg(White);
            style = style.bg(White);
        }


        style
    }
}





impl Widget for MineField {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let col_constraints = (0..self.columns()).map(|_| Constraint::Fill(1));
        let row_constraints = (0..self.rows()).map(|_| Constraint::Fill(1));
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {

            let cell_layout = Layout::default()
                                        .direction(ratatui::layout::Direction::Vertical)
                                        .constraints(
                                            vec![
                                                Constraint::Percentage(20),
                                                Constraint::Fill(1),
                                                Constraint::Percentage(10)
                                            ]
                                        ).split(cell);
            
            let cell_style = self.cell_style(i);

            // render the top and bottom empty portions as solid blocks 
            // with teh matching style of the cell
            Block::new().style(cell_style).render(cell_layout[0], buf);
            Block::new().style(cell_style).render(cell_layout[2], buf);

            let cell_text = if self.check_neighbors_index(i) != 0 && self.is_revealed_index(i){
                format!("{}", self.check_neighbors_index(i))
            } else {
                "".to_owned()
            };

            Paragraph::new(cell_text)
                        .style(cell_style)
                        .centered()
                        .render(cell_layout[1], buf);

        }
    }
}




