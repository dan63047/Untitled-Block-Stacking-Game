use std::time::SystemTime;

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

pub struct TGM {
    memory: Vec<usize>,
    seed: u32
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

impl TGM {
    fn random(n: u32) -> u32{
        n.wrapping_mul(0x41c64e6d).wrapping_add(12345) & 0xffffffff
    }

    fn read(&mut self) -> u32 {
        self.seed = TGM::random(self.seed);
        (self.seed >> 10) & 0x7fff
    }
}

impl Randomizer for TGM {
    fn populate_next(&mut self, pieces_data: &PiecesData, board_width: isize, board_height: isize) -> Vec<Piece> {
        let mut b = 0;
        for _ in 0..4{
            b = self.read() % 7;
            if !self.memory.contains(&(b as usize)){break;}
            b = self.read() % 7;
        }
        self.memory.pop();
        self.memory.insert(0, b as usize);
        vec![Piece::create(pieces_data, b as usize, board_width, board_height)]
    }

    fn create() -> Self where Self: Sized {
        let mut amazon_prime = TGM {
            memory: vec![],
            seed: match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => n.as_secs().try_into().unwrap(),
                Err(_) => panic!("CLOCK???? mclock ⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰⏰"),
            }
        };
        let mut b = 0;
        while b == 0 || b == 6 || b == 4 {
            b = amazon_prime.read() as usize % 7;
        }
        amazon_prime.memory = vec![b, 0, 0, 0];
        amazon_prime
    }
}