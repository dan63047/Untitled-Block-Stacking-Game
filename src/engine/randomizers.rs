use rand::seq::SliceRandom;
use rand::thread_rng;

use super::{rotation_systems::PiecesData, resources::Piece};

pub trait Randomizer{
    fn populate_next(&self, pieces_data: &PiecesData) -> Vec<Piece>;
}

pub struct Bag {}

impl Randomizer for Bag {
    fn populate_next(&self, pieces_data: &PiecesData) -> Vec<Piece> {
        let mut bag = vec![];
        let mut id: usize = 0;
        for _ in &pieces_data.pieces{
            bag.insert(id, Piece::create(pieces_data, id));
            id += 1;
        }
        let mut rng = thread_rng();
        bag.shuffle(&mut rng);
        bag
    }
}