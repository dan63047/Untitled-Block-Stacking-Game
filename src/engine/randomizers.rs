use rand::seq::SliceRandom;
use rand::thread_rng;

use super::{rotation_systems::PiecesData, resources::Piece};

pub trait Randomizer{
    fn populate_next(&self, pieces_data: PiecesData) -> Vec<Piece>;
}

pub struct Bag {}

impl Randomizer for Bag {
    fn populate_next(&self, pieces_data: PiecesData) -> Vec<Piece> {
        let mut bag = vec![];
        let mut id: usize = 0;
        for _ in pieces_data.pieces{
            let final_position = (3+pieces_data.spawn_offsets[id].0, 20+pieces_data.spawn_offsets[id].1);
            let element = Piece { id: id, position: final_position, rotation: 0 };
            id += 1;
            bag.insert(0, element);
        }
        let mut rng = thread_rng();
        bag.shuffle(&mut rng);
        bag
    }
}