/*
Minefield is the class
that holds the state of the board during the game

the board is an array that represents a 
2d grid of squares 

each square is either unknown, bomb, flag, or number
*/



use rand::distr::Uniform;

use color_eyre::Result;


#[derive(Clone, Copy)]
pub enum SquareState {
    Safe,
    Bomb,
}


pub struct MineField {
    size: [usize; 2],
    state: Vec<SquareState>,
    num_bombs: u8,
    area: usize,
}

impl MineField {

    pub fn new(self: &mut Self, width: usize, height: usize, num_bombs: usize) -> Result<()>{

        // set the minefield state
        // based on the set up stuff


        self.size = [width, height];
        self.area = width*height;

        self.state = vec![SquareState::Safe; self.area.into()];


        // set the bomb indices
        let bomb_spots = rand::seq::index::sample(&mut rand::rng(), self.area, num_bombs);
        for index in bomb_spots {
            self.state[index] = SquareState::Bomb;
        }
        Ok(())
    }

    pub fn check_square(self: Self, row_index: usize, col_index: usize) -> SquareState{
        // get the type the square is 

        let index:usize = (row_index * self.size[0]) + col_index;
        
        self.state[index]
    }




}


