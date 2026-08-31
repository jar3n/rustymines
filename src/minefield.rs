/*
Minefield is the class
that holds the state of the board during the game

the board is an array that represents a 
2d grid of squares 

each square is either unknown, bomb, flag, or number
*/

use std::option::Option;

use rand::seq::IteratorRandom;


#[derive(Clone)]
pub struct MineField {
    columns: usize,
    rows: usize,
    state: Vec<Option<i8>>,
    flagged_spots: Vec<usize>,
    revealed_spots: Vec<usize>,
    num_bombs: usize,
    area: usize,
    bombs_set: bool,
    selected_spot: usize
}
impl Default for MineField {
    fn default() -> Self {
        Self {
            columns: 3,
            rows: 3,
            state: vec![None; 9],
            num_bombs: 0,
            area: 9,
            flagged_spots: vec![],
            revealed_spots: vec![],
            bombs_set: true,
            selected_spot: 0,
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
            flagged_spots: vec![],
            revealed_spots: vec![],
            bombs_set: false,
            selected_spot: 0,
        };

        field.area = field.columns*field.rows;

        field.state = vec![None; field.area.into()];

        field
        
    }

    fn place_bombs(self: &mut Self, revealed_index: usize) {
        // instead of placing bombs to start
        // place them after the user makes the first move

        // get all possible bomb indicies
        let mut possible_indexes: Vec<usize> = (0..self.area).into_iter().collect();

        possible_indexes.remove(possible_indexes.iter().position(|r|*r == revealed_index).unwrap());

        // randomly pick spots from the possible indexes
        // then remove from the list of options
        for _ in 0..self.num_bombs {
            let bomb_spot = *possible_indexes.iter().choose(&mut rand::rng()).unwrap();

            self.state[bomb_spot] = Some(-1);

            possible_indexes.remove(possible_indexes.iter().position(|r| *r == bomb_spot).unwrap());
        }

        self.bombs_set = true;

    }

    pub fn is_bomb(self: &Self, row_index: usize, col_index: usize) -> bool{
        // check if square is bomb

        let index:usize = (row_index * self.columns) + col_index;
        
        self.state[index] == Some(-1)
    }

    pub fn is_selected_bomb(self: &Self) -> bool {
        self.state[self.selected_spot] == Some(-1)
    }

    pub fn update_selected_spot(self: &mut Self, row_delta: isize, column_delta: isize) {


        // the deltas are either -1, 0, 1

        let rows = self.rows as isize;
        let area = self.area as isize;
        let curr_spot = self.selected_spot as isize;

        if let Some(new_spot) = curr_spot.checked_add(row_delta*rows) && 
                                                            new_spot < area {
            if let Some(new_spot) = new_spot.checked_add(column_delta) && new_spot < area {
                if new_spot > -1 {
                    self.selected_spot = new_spot as usize;
                }
            }
        }
    }

    pub fn selected_square(self: &Self) -> usize {
        self.selected_spot
    }

    pub fn reveal_square(self: &mut Self) {

        let index:usize = self.selected_spot;


        if !self.bombs_set {
            self.place_bombs(index);
        }

        if self.state[index] == None || self.state[index] == Some(-1){
            if self.state[index] != Some(-1) {
                self.state[index] = Some(self.check_neighbors_index(index));
            }

            self.revealed_spots.push(index);

            // recursion for reavaling all
            // spots with neighboring 
            // this spot which has no bombs
            if self.state[index] == Some(0) {

                for neighbor in self.get_neighbors_index(index) {

                    self.reveal_square_index(neighbor);
                }
            }
        }

        // unflag the square when revealed
        if self.is_flagged_index(index) {
            self.unflag_square_index(index);
        }

    }

    fn reveal_square_index(self: &mut Self, index:usize) {
        // helper function to do recursive reveal

        if self.state[index] == None || self.state[index] == Some(-1){
            if self.state[index] != Some(-1) {
                self.state[index] = Some(self.check_neighbors_index(index));
            }

            self.revealed_spots.push(index);

            // recursion for reavaling all
            // spots with no neighboring bombs
            if self.state[index] == Some(0) {

                for neighbor in self.get_neighbors_index(index) {

                    self.reveal_square_index(neighbor);

                }

                
            }

            // unflag the square when revealed
            if self.flagged_spots.contains(&index) {
                let index_of_index = self.flagged_spots.iter().position(|r| *r == index).unwrap();
                self.flagged_spots.remove(index_of_index);
            }  
        }

    }

    pub fn num_bombs(self: &Self) -> usize  {
        self.num_bombs
    }

    pub fn flag_selected_square(self: &mut Self) {
        if !self.flagged_spots.contains(&self.selected_spot) && !self.revealed_spots.contains(&self.selected_spot) {
            self.flagged_spots.push(self.selected_spot);
        }
    }

    fn unflag_square_index(self: &mut Self, index: usize){

        if self.flagged_spots.contains(&index) {
            let index_of_index = self.flagged_spots.iter().position(|r| *r == index).unwrap();

            self.flagged_spots.remove(index_of_index);
        }
    }

    pub fn unflag_selected_square(self: &mut Self) {
        self.unflag_square_index(self.selected_spot);
    }

    pub fn is_selected_flagged(self: &mut Self) -> bool{
        self.is_flagged_index(self.selected_spot)
    }

    fn get_neighbors_index(self: &Self, square_index:usize) -> Vec<usize> {
        // neighbor edge cases
        // are border cells
        // remove the invalid elements
        // when its a border cell
        let top_border = 0..self.columns;
        let bottom_border = (self.area - self.columns)..self.area;

        let left_border: Vec<usize> = (0..(self.area - self.columns+1)).step_by(self.columns).collect();
        let right_border: Vec<usize> = ((self.columns-1)..(self.area)).step_by(self.columns).collect();

        let mut neighbors = vec![];
        
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


    pub fn check_neighbors_index(self: &Self, index: usize) -> i8 {
        let neighbors = self.get_neighbors_index(index);

        let mut count = 0;

        for neighbor in neighbors {
            let neighbor_val = self.state[neighbor];
            if neighbor_val == Some(-1){
                count += 1;
            }
        }

        count
    }

    pub fn num_revealed_spots(self: &Self) -> usize {
        self.revealed_spots.len()
    }

    pub fn area(self: &Self) -> usize {
        self.area
    }

    pub fn rows(self: &Self) -> usize {
        self.rows
    }

    pub fn columns(self: &Self) -> usize {
        self.columns
    }

    pub fn is_revealed_index(self: &Self, index:usize) -> bool {
        self.revealed_spots.contains(&index)
    }

    pub fn is_flagged_index(self: &Self, index: usize) -> bool {
        self.flagged_spots.contains(&index)
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

        // center index testing
        let act_neighbors = field.get_neighbors_index(4);
        let exp_neighbors: Vec<usize> = vec![0,1,2,3,5,6,7,8];

        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();


    }


    #[test]
    fn test_top_left_corner_neighbors() {
        let field = MineField::default();

        // center index testing
        let act_neighbors = field.get_neighbors_index(0);
        let exp_neighbors: Vec<usize> = vec![1,3,4];

        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

       #[test]
    fn test_top_right_corner_neighbors() {
        let field = MineField::default();

        // center index testing
        let act_neighbors = field.get_neighbors_index(2);
        let exp_neighbors: Vec<usize> = vec![1,4,5];

        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_bottom_left_corner_neighbors() {
        let field = MineField::default();


        // center index testing
        let act_neighbors = field.get_neighbors_index(6);
        let exp_neighbors: Vec<usize> = vec![3,4,7];

        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_bottom_right_corner_neighbors() {
        let field = MineField::default();

        // center index testing
        let act_neighbors = field.get_neighbors_index(8);
        let exp_neighbors: Vec<usize> = vec![4,5,7];

        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_top_side_neighbors() {
        let field = MineField::default();

        // center index testing
        let act_neighbors = field.get_neighbors_index(1);
        let exp_neighbors: Vec<usize> = vec![0,2,3,4,5];

        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_left_side_neighbors() {
        let field = MineField::default();

        // center index testing
        let act_neighbors = field.get_neighbors_index(3);
        let exp_neighbors: Vec<usize> = vec![0,1,4,6,7];

        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_right_side_neighbors() {
        let field = MineField::default();


        // center index testing
        let act_neighbors = field.get_neighbors_index(5);
        let exp_neighbors: Vec<usize> = vec![1,2,4,7,8];

        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }

    #[test]
    fn test_bottom_side_neighbors() {
        let field = MineField::default();

        // center index testing
        let act_neighbors = field.get_neighbors_index(7);
        let exp_neighbors: Vec<usize> = vec![3,4,5,6,8];

        assert_neighbors(&act_neighbors, &exp_neighbors);
        print_separator();
    }


    #[test]
    fn test_check_square_bomb() {
        let field = set_bomb_location(1, 1);

        let is_bomb = field.is_bomb(1,1);

        assert!(is_bomb);
    }

    #[test]
    fn test_check_square_no_bomb() {
        let field = set_bomb_location(1,1);
        let is_bomb = field.is_bomb(0,0);

        assert!(!is_bomb);
    }

    #[test]
    fn test_check_neighbors_center_one() {
        let field = set_bomb_location(1, 1);

        // bomb in center means all non bomb squares
        // have one bomb neighbor
        // since default is 3x3
        for spot in 0..field.area {
            if spot == 4 {
                // skip the check on the spot with the bomb
                continue
            }
                let bomb_count = field.check_neighbors_index(spot);
                
                assert_eq!(bomb_count, 1);
        }
    }

    #[test]
    fn test_check_neighbors_two_bombs(){
        let mut field = set_bomb_location(0, 1);

        add_bomb_location(&mut field, 2, 1);

        let two_bomb_neighbors: Vec<usize> = vec![
                    3,
                    4,
                    5
                ];

        for spot in two_bomb_neighbors {
            let bomb_count = field.check_neighbors_index(spot);
            assert_eq!(bomb_count,2);

        }

    }

    #[test]
    fn test_new_minefield_has_no_bombs() {
        let new_field = MineField::new(3, 3, 3);

        for square in 0..new_field.area {
            assert_eq!(new_field.state[square], None);
        }
    }

}
