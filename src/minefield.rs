/*
Minefield is the class
that holds the state of the board during the game

the board is an array that represents a 
2d grid of squares 

each square is either unknown, bomb, flag, or number
*/

use std::option::Option;


#[derive(Clone)]
pub struct MineField {
    columns: usize,
    rows: usize,
    state: Vec<Option<i8>>,
    num_bombs: usize,
    bomb_spots: Vec<usize>,
    area: usize,
}
impl Default for MineField {
    fn default() -> Self {
        Self {
            columns: 3,
            rows: 3,
            state: vec![None; 9],
            bomb_spots: vec![0; 9],
            num_bombs: 0,
            area: 9,
        }
    }
}

impl MineField {

    pub fn new(columns: usize, rows: usize, num_bombs: usize) -> Self{

        // set the minefield state
        // based on the set up stuff
        let mut field = MineField {
            columns: columns,
            rows: rows,
            state: vec![None],
            num_bombs: num_bombs,
            area: 1,
            bomb_spots: vec![],
        };

        field.area = field.columns*field.rows;

        field.state = vec![None; field.area.into()];


        // set the bomb indices
        field.bomb_spots = rand::seq::index::sample(&mut rand::rng(), field.area, field.num_bombs).into_vec();
        
        field
        
    }

    pub fn check_square(self: &Self, row_index: usize, col_index: usize) -> Option<i8>{
        // get the type the square is 

        let index:usize = (row_index * self.columns) + col_index;
        
        self.state[index]
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
        } else if left_border.contains(&square_index) {
            // just left border not a corner
            neighbors.push(square_index + 1);
            neighbors.push(square_index - self.columns);
            neighbors.push(square_index - self.columns + 1);
            neighbors.push(square_index + self.columns);
            neighbors.push(square_index + self.columns + 1);

        } else if right_border.contains(&square_index) {
            // just right border not a corner
            neighbors.push(square_index - self.columns);
            neighbors.push(square_index - self.columns -1);
            neighbors.push(square_index + self.columns);
            neighbors.push(square_index - 1);
            neighbors.push(square_index + self.columns - 1);
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

    pub fn check_neighbors(self: &mut Self, row_index: usize, col_index: usize) -> i8 {

        // count the number of bombs adjacent to the current square

        let index = (row_index * self.columns) + col_index;

        match self.state[index] {
            Some(count) => count,
            None => {
                        let neighbors = self.get_neighbors(row_index, col_index);

                        let mut count = 0;

                        for neighbor in neighbors {
                            let neighbor_val = self.state[neighbor];
                            if neighbor_val == Some(-1){
                                count += 1;
                            }
                        }

                        // update the square value
                        self.state[index] = Some(count);

                        count
                    }
                }
    }



    pub fn rows(self: &Self) -> usize {
        self.rows
    }

    pub fn columns(self: &Self) -> usize {
        self.columns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test set up and helper functions
    fn assert_neighbors(act_neighbors: &Vec<usize>, exp_neighbors: &Vec<usize>) {

        println!("Number of Actual Neighbors: {}",act_neighbors.len());
        println!("Number of Expected Neighbors: {}",exp_neighbors.len());

        println!("Actual Neighbors: {:?}", act_neighbors);
        println!("Expected Neighbors: {:?}", exp_neighbors);


        // compare number of neighbors
        assert_eq!(act_neighbors.len(), exp_neighbors.len());

        for exp_neighbor in exp_neighbors {
            assert!(act_neighbors.contains(exp_neighbor));
        }
    }

    fn print_separator() {
        println!("----------------------");
    }

    struct TestNeighborIndexes {
        center_index: (usize, usize),
        corner_indexes: Vec<(usize,usize)>,
        side_indexes: Vec<(usize,usize)>,
    }
    impl Default for TestNeighborIndexes {
        fn default() -> Self {
            Self {
                center_index: (1,1),
                corner_indexes:  vec![
                    (0,0),
                    (0,2),
                    (2,0),
                    (2,2)
                ],
                side_indexes: vec![
                    (0,1),
                    (1,0),
                    (1,2),
                    (2,1),
                ],
            }
        }
    }

    fn set_bomb_location(row:usize, column:usize) -> MineField {
        let mut field = MineField::default();

        let state_index = (row * field.columns) + column;

        field.state[state_index] = Some(-1);

        field.num_bombs = 1;

        field
    }

    fn add_bomb_location(field:&mut MineField, row:usize, column:usize) {
        
        let state_index = (row * field.columns) + column;

        field.state[state_index] = Some(-1);

        field.num_bombs += 1;
    }
   
    // tests
    #[test]
    fn test_get_neighbors_center() {
        let field = MineField::default();

        // index with 8 neighbors
        let coords = TestNeighborIndexes::default().center_index;
        

        // center index testing
        let act_neighbors = field.get_neighbors(coords.0, coords.1);
        let exp_neighbors: Vec<usize> = vec![0,1,2,3,5,6,7,8];


        println!("Testing Center Index, {:?}", coords);
        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();


    }


    #[test]
    fn test_top_left_corner_neighbors() {
        let field = MineField::default();

        let coords = TestNeighborIndexes::default().corner_indexes[0];

        // center index testing
        let act_neighbors = field.get_neighbors(coords.0, coords.1);
        let exp_neighbors: Vec<usize> = vec![1,3,4];

        println!("Testing Top Left Corner Index, {:?}", coords);
        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

       #[test]
    fn test_top_right_corner_neighbors() {
        let field = MineField::default();

        let coords = TestNeighborIndexes::default().corner_indexes[1];

        // center index testing
        let act_neighbors = field.get_neighbors(coords.0, coords.1);
        let exp_neighbors: Vec<usize> = vec![1,4,5];

        println!("Testing Top Right Corner Index, {:?}", coords);
        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_bottom_left_corner_neighbors() {
        let field = MineField::default();

        let coords = TestNeighborIndexes::default().corner_indexes[2];

        // center index testing
        let act_neighbors = field.get_neighbors(coords.0, coords.1);
        let exp_neighbors: Vec<usize> = vec![3,4,7];

        println!("Testing Bottom Left Corner Index, {:?}", coords);
        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_bottom_right_corner_neighbors() {
        let field = MineField::default();

        let coords = TestNeighborIndexes::default().corner_indexes[3];

        // center index testing
        let act_neighbors = field.get_neighbors(coords.0, coords.1);
        let exp_neighbors: Vec<usize> = vec![4,5,7];

        println!("Testing Bottom Right Corner Index, {:?}", coords);
        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_top_side_neighbors() {
        let field = MineField::default();

        let coords = TestNeighborIndexes::default().side_indexes[0];

        // center index testing
        let act_neighbors = field.get_neighbors(coords.0, coords.1);
        let exp_neighbors: Vec<usize> = vec![0,2,3,4,5];

        println!("Testing Top Side Index, {:?}", coords);
        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_left_side_neighbors() {
        let field = MineField::default();

        let coords = TestNeighborIndexes::default().side_indexes[1];

        // center index testing
        let act_neighbors = field.get_neighbors(coords.0, coords.1);
        let exp_neighbors: Vec<usize> = vec![0,1,4,6,7];

        println!("Testing Right Side Index, {:?}", coords);
        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_right_side_neighbors() {
        let field = MineField::default();

        let coords = TestNeighborIndexes::default().side_indexes[2];

        // center index testing
        let act_neighbors = field.get_neighbors(coords.0, coords.1);
        let exp_neighbors: Vec<usize> = vec![1,2,4,7,8];

        println!("Testing Right Side Index, {:?}", coords);
        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_bottom_side_neighbors() {
        let field = MineField::default();

        let coords = TestNeighborIndexes::default().side_indexes[3];

        // center index testing
        let act_neighbors = field.get_neighbors(coords.0, coords.1);
        let exp_neighbors: Vec<usize> = vec![3,4,5,6,8];

        println!("Testing Bottom Side Index, {:?}", coords);
        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }


    #[test]
    fn test_check_square_bomb() {
        let field = set_bomb_location(1, 1);

        let is_bomb = field.check_square(1,1);

        assert!(is_bomb);
    }

    #[test]
    fn test_check_square_no_bomb() {
        let field = set_bomb_location(1,1);
        let is_bomb = field.check_square(0,0);

        assert!(!is_bomb);
    }

    #[test]
    fn test_check_neighbors_center_one() {
        let mut field = set_bomb_location(1, 1);

        // bomb in center means all non bomb squares
        // have one bomb neighbor
        // since default is 3x3
        for row in 0..field.rows {
            for column in 0..field.columns {
                if row == 1 && column ==1 {
                    continue;
                }

                let bomb_count = field.check_neighbors(row, column);
                
                assert_eq!(bomb_count, 1);

            }
        }
    }

    #[test]
    fn test_check_neighbors_two_bombs(){
        let mut field = set_bomb_location(0, 1);

        add_bomb_location(&mut field, 2, 1);


        for row in 0..field.rows {
            for column in 0..field.columns {
                let coords: (usize, usize) = (row, column);

                let two_bomb_neighbors: Vec<(usize, usize)> = vec![
                    (1,0),
                    (1,1),
                    (1,2),
                ];

                let bomb_count = field.check_neighbors(coords.0, coords.1);

                if two_bomb_neighbors.contains(&coords) {
                    assert_eq!(bomb_count,2);
                } else if field.check_square(coords.0, coords.1) {
                    // ignore if its a bomb square
                    continue;
                } 
                else {
                    assert_eq!(bomb_count, 1);
                }
            }
        }

    }

}
