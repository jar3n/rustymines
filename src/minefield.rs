/*
Minefield is the class
that holds the state of the board during the game

the board is an array that represents a 
2d grid of squares 

each square is either unknown, bomb, flag, or number
*/



use std::collections::HashSet;

use color_eyre::Result;
use rand::random_iter;


#[derive(Clone, Copy)]
pub enum SquareState {
    Safe,
    Bomb,
}


pub struct MineField {
    size: [usize; 2],
    state: Vec<SquareState>,
    num_bombs: u8,
}

impl MineField {

    pub fn new(self: &mut Self, width: usize, height: usize, num_bombs: u8) -> Result<()>{

        // set the minefield state
        // based on the set up stuff


        self.size = [width, height];

        self.state = vec![SquareState::Safe; (width*height).into()];

        let bomb_spots: HashSet<u32> = random_iter().take(num_bombs.into()).collect();

        // set the bomb indices
        for index in 0..self.state.len() {
            let idx = index as u32;
            if bomb_spots.contains(&idx) {
                self.state[index] = SquareState::Bomb;
            }
        }
        Ok(())
    }

    pub fn check_square(self: Self, row_index: usize, col_index: usize) -> SquareState{
        // get the type the square is 

        let index:usize = (row_index * self.size[0]) + col_index;
        
        self.state[index]
    }




}


