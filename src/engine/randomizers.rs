use rand::{seq::SliceRandom, random};
use rand::thread_rng;

use super::{rotation_systems::PiecesData, resources::Piece};

pub trait Randomizer{
    fn create() -> Self where Self: Sized;
    fn populate_next(&mut self, pieces_data: &PiecesData, board_width: isize, board_height: isize) -> Vec<Piece>;
}

pub struct Bag {}

pub struct BagX2 {}

pub struct RandomWithoutDirectRepetition {
    memory: usize
}

impl Randomizer for Bag {
    fn populate_next(&mut self, pieces_data: &PiecesData, board_width: isize, board_height: isize) -> Vec<Piece> {
        let mut bag = vec![];
        let mut id: usize = 0;
        for _ in &pieces_data.pieces{
            bag.insert(id, Piece::create(pieces_data, id, board_width, board_height));
            id += 1;
        }
        let mut rng = thread_rng();
        bag.shuffle(&mut rng);
        bag
    }

    fn create() -> Self where Self: Sized {
        Bag {  }
    }
}

impl Randomizer for BagX2 {
    fn populate_next(&mut self, pieces_data: &PiecesData, board_width: isize, board_height: isize) -> Vec<Piece> {
        let mut bag = vec![];
        let mut id: usize = 0;
        for _ in &pieces_data.pieces{
            bag.insert(id, Piece::create(pieces_data, id, board_width, board_height));
            bag.insert(id, Piece::create(pieces_data, id, board_width, board_height));
            id += 1;
        }
        let mut rng = thread_rng();
        bag.shuffle(&mut rng);
        bag
    }

    fn create() -> Self where Self: Sized {
        BagX2 {  }
    }
}

impl Randomizer for RandomWithoutDirectRepetition {
    fn populate_next(&mut self, pieces_data: &PiecesData, board_width: isize, board_height: isize) -> Vec<Piece> {
        let random_number = random::<usize>() % pieces_data.pieces.len();
        if random_number != self.memory {
            self.memory = random_number;
            vec![Piece::create(pieces_data, random_number, board_width, board_height)]
        }else{
            let random_number = random::<usize>() % pieces_data.pieces.len();
            self.memory = random_number;
            vec![Piece::create(pieces_data, random_number, board_width, board_height)]
        }
        
    }

    fn create() -> Self where Self: Sized {
        RandomWithoutDirectRepetition { memory: 65535 }
    }
}