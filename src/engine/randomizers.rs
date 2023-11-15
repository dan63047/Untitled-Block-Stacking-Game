use rand::{seq::SliceRandom, random};
use rand::thread_rng;

use super::{rotation_systems::PiecesData, resources::Piece};

pub trait Randomizer{
    fn populate_next(&self, pieces_data: &PiecesData, board_width: isize, board_height: isize) -> Vec<Piece>;
}

pub struct Bag {}

pub struct RandomWithoutDirectRepetition {}

impl Randomizer for Bag {
    fn populate_next(&self, pieces_data: &PiecesData, board_width: isize, board_height: isize) -> Vec<Piece> {
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
}

impl Randomizer for RandomWithoutDirectRepetition {
    fn populate_next(&self, pieces_data: &PiecesData, board_width: isize, board_height: isize) -> Vec<Piece> {
        let random_number = random::<usize>() % pieces_data.pieces.len();
        vec![Piece::create(pieces_data, random_number, board_width, board_height)]
    }
}