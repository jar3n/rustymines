/*
Minefield is the class
that holds the state of the board during the game

the board is an array that represents a 
2d grid of squares 

each square is either unknown, bomb, flag, or number
*/


#[derive(Clone)]
pub struct MineField {
    columns: usize,
    rows: usize,
    state: Vec<i8>,
    num_bombs: usize,
    area: usize,
}

impl MineField {

    pub fn new(columns: usize, rows: usize, num_bombs: usize) -> Self{

        // set the minefield state
        // based on the set up stuff
        let mut field = MineField {
            columns: columns,
            rows: rows,
            state: vec![-1],
            num_bombs: num_bombs,
            area: 1,
        };

        field.area = field.columns*field.rows;

        field.state = vec![0; field.area.into()];


        // set the bomb indices
        let bomb_spots = rand::seq::index::sample(&mut rand::rng(), field.area, field.num_bombs);
        for index in bomb_spots {
            field.state[index] = -1;
        }

        field
        
    }

    pub fn check_square(self: &Self, row_index: usize, col_index: usize) -> bool{
        // get the type the square is 

        let index:usize = (row_index * self.columns) + col_index;
        
        match self.state[index] {
            -1 => true,
            _ => false,
        }
    }

    fn get_neighbors(self: &Self, row_index: usize, col_index: usize) -> Vec<usize> {
        
        let square_index: usize = (row_index*self.columns) + col_index;

        let mut neighbors = vec![];
        
        // neighbor edge cases
        // are border cells
        // remove the invalid elements
        // when its a border cell
        let top_border = 0..self.columns;
        let bottom_border = (self.area - self.columns)..self.area;

        let left_border: Vec<usize> = (0..(self.area - self.columns+1)).step_by(self.columns).collect();
        let right_border: Vec<usize> = ((self.columns-1)..(self.area)).step_by(self.columns).collect();
        
        if top_border.contains(&square_index){
            if left_border.contains(&square_index) {
                // top left corner
                neighbors.push(square_index + 1);
                neighbors.push(square_index + self.columns);
                neighbors.push(square_index + self.columns + 1);
            } else if right_border.contains(&square_index) {
                // top right corner
                neighbors.push(square_index - 1);
                neighbors.push(square_index + self.columns);
                neighbors.push(square_index + self.columns - 1);
            } else {
                // not a corner just in the top border
                neighbors.push(square_index + 1);
                neighbors.push(square_index + self.columns);
                neighbors.push(square_index + self.columns + 1);
                neighbors.push(square_index - 1);
                neighbors.push(square_index + self.columns - 1);
            }
        } else if bottom_border.contains(&square_index) {
            if left_border.contains(&square_index) {
                // bottom left corner
                neighbors.push(square_index + 1);
                neighbors.push(square_index - self.columns);
                neighbors.push(square_index - self.columns + 1);
            } else if right_border.contains(&square_index) {
                // bottom right corner
                neighbors.push(square_index - self.columns);
                neighbors.push(square_index - self.columns - 1);
                neighbors.push(square_index - 1);
            } else {
                // not a corner just in the bottom border
                neighbors.push(square_index + 1);
                neighbors.push(square_index - self.columns);
                neighbors.push(square_index - self.columns + 1);
                neighbors.push(square_index - 1);
                neighbors.push(square_index - self.columns - 1);
            }
        } else {
            // not an edge case
            // add all the neighbors
            neighbors.push(square_index + 1);
            neighbors.push(square_index - self.columns);
            neighbors.push(square_index - self.columns + 1);
            neighbors.push(square_index - 1);
            neighbors.push(square_index - self.columns - 1);
            neighbors.push(square_index + self.columns);
            neighbors.push(square_index + self.columns + 1);
            neighbors.push(square_index + self.columns - 1);

        }

        neighbors


    }

    pub fn check_neighbors(self: &Self, row_index: usize, col_index: usize) -> i8 {

        // count the number of bombs adjacent to the current square

        let neighbors = self.get_neighbors(row_index, col_index);

        let mut count = 0;

        for neighbor in neighbors {
            let neighbor_val = self.state[neighbor];
            if neighbor_val == -1 {
                count += 1;
            }
        }
        
        count

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_index() {
        let field = MineField::new(3, 3, 1);

        // valid index


    }
}
