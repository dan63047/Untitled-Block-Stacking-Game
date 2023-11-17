use std::mem::swap;

use bevy::prelude::*;

use super::{rotation_systems::{PiecesData, ROTATION_SYSTEMS, LockDelayMode}, components::Mino, randomizers::{Randomizer, Bag}};

#[derive(Clone, Copy)]
pub struct Piece{
    pub id: usize,
    pub color: Color,
    pub position: (isize, isize),
    pub rotation: usize
}

impl Piece {
    pub fn create(pieces_data: &PiecesData, id: usize, board_width: isize, board_height: isize) -> Piece{
        let final_position = (
            board_width/2 - 2 + pieces_data.spawn_offsets[id].0,
            board_height + pieces_data.height_offset + pieces_data.spawn_offsets[id].1
        );
        Piece { id: id, color: pieces_data.colours[id], position: final_position, rotation: 0 }
    }
}

pub struct Board{
    pub width: u8,
    pub height: u8,
    pub buffer_height: u8,
    pub show_grid: bool,
    pub show_shadow: bool,
    pub show_next: u8,
    // X axis - from left to right; Y axis - from bottom to top (board[y][x])
    pub board: Vec<Vec<Option<Mino>>>
}

impl Board{
    pub fn create(width: u8, height: u8, buffer_height: u8, show_grid: bool, show_shadow: bool, show_next: u8) -> Board {
        let board: Vec<Vec<Option<Mino>>> = vec![vec![None; width as usize]; (height+buffer_height) as usize];
        Board { width: width, height: height, buffer_height: buffer_height, show_grid: show_grid, show_shadow: show_shadow, show_next: show_next, board: board }
    }

    pub fn clear_full_lines(&mut self) {
        let mut lines_cleared: usize = 0;
        for row in 0..self.board.len(){
            if self.board[row-lines_cleared].iter().all(|l| l.is_some()){
                self.board.remove(row-lines_cleared);
                let empty_row: Vec<Option<Mino>> = vec![None; self.width as usize];
                self.board.push(empty_row);
                lines_cleared += 1;
            }
        }
    }
}

pub struct Handling{
    pub das: f32, // ms
    pub arr: f32, // ms
    pub sdf: f32, // gravity*sdf = soft drop
    pub das_left: f32, // ms
    pub arr_left: f32, // ms
    pub sdf_active: bool,
    pub active_left: bool,
    pub active_right: bool,
    pub direction: i8 // -1 - left, 1 - right, 0 - none
}

impl Handling {
    pub fn create(das: f32, arr: f32, sdf: f32) -> Handling{
        Handling { das: das, arr: arr, sdf: sdf, das_left: das, arr_left: arr, sdf_active:false, active_left: false, active_right: false, direction: 0}
    }

    pub fn movement_key_pressed(&mut self, left: bool, right: bool){
        if left {
            self.active_left = left;
            self.direction = -1;
        }
        if right {
            self.active_right = right;
            self.direction = 1;
        }
        self.das_left = self.das;
    }

    pub fn movement_key_released(&mut self, left: bool, right: bool){
        if left {
            self.active_left = !left;
        }
        if right {
            self.active_right = !right;
        }
        if self.active_left {
            self.direction = -1;
        }
        if self.active_right {
            self.direction = 1;
        }
        if !self.active_left && !self.active_right{
            self.arr_left = self.arr;
            self.das_left = self.das;
            self.direction = 0;
        }
    }

    pub fn movement_tick(&mut self, delta: f32) -> i8 {
        if !self.active_left && !self.active_right {
            return 0;
        } 
        if self.das_left > 0.0 {
            self.das_left -= delta;
            if self.das_left <= 0.0 {
                self.arr_left += self.das_left;
                self.das_left = 0.0;
                return self.direction;
            }else{
                return  0;
            }
        }else{
            self.arr_left -= delta;
            if self.arr_left <= 0.0 {
                self.arr_left += self.arr;
                return self.direction;
            }else {
                return 0;
            }
        }
    }
}

#[derive(Resource)]
pub struct Engine {
    pub current_piece: Option<Piece>,
    pub board: Board,
    pub handling: Handling,
    pub rotation_system: PiecesData,
    pub randomizer: Box<dyn Randomizer + Sync + Send>,
    pub next_queue: Vec<Piece>,
    pub hold: Option<Piece>,
    pub can_hold: bool, // anti-abuse
    pub hold_enabled: bool, // game rule
    pub g: f32,
    pub g_bucket: f32,
    pub lock_delay: u8,
    pub lock_delay_left: u8,
    pub lock_delay_resets: u8,
    pub lock_delay_resets_left: u8,
    pub lock_delay_active: bool,
    pub need_to_lock: bool, // when lock resets ended
}

impl Default for Engine {
    fn default() -> Engine {
        Engine {
            current_piece: None,
            board: Board::create(10, 20, 20, true, true, 3),
            handling: Handling::create(200.0, 33.0, 20.0),
            rotation_system: ROTATION_SYSTEMS["SRS"].clone(),
            next_queue: vec![],
            hold: None,
            can_hold: true,
            hold_enabled: true,
            g: 1.0/60.0,
            g_bucket: 0.0,
            lock_delay: 30,
            lock_delay_left: 30,
            lock_delay_resets: 15,
            lock_delay_resets_left: 15,
            lock_delay_active: false,
            need_to_lock: false,
            randomizer: Box::new(Bag{}),
        }
    }
}

impl Engine {
    fn from_next_to_current(&mut self){
        if self.next_queue.len() <= self.board.show_next as usize {
            self.next_queue.append(&mut self.randomizer.populate_next(&self.rotation_system, self.board.width as isize, self.board.height as isize));
        }
        self.current_piece = self.next_queue.first().copied();
        self.next_queue.remove(0);
    }

    pub fn init(&mut self, rotation_system: &str, randomizer: Box<dyn Randomizer + Sync + Send>){
        self.rotation_system = ROTATION_SYSTEMS[rotation_system].clone();
        self.randomizer = randomizer;
        while self.next_queue.len() <= self.board.show_next as usize  {
            self.next_queue.append(&mut self.randomizer.populate_next(&self.rotation_system, self.board.width as isize, self.board.height as isize));
        }
        self.from_next_to_current();
    }

    pub fn spawn_sequence(&mut self) -> bool {
        self.from_next_to_current();
        if !self.position_is_valid(self.current_piece.as_ref().unwrap().position, self.current_piece.as_ref().unwrap().rotation){
            return false;
        }
        self.can_hold = true;
        if self.g >= 20.0 { self.current_piece.as_mut().unwrap().position.1 = self.lowest_point_under_current_piece() }
        true
    }

    pub fn hold_current_piece(&mut self) -> bool {
        if  !self.hold_enabled || !self.can_hold {
            return false;
        }
        self.current_piece.as_mut().unwrap().rotation = 0;
        self.current_piece.as_mut().unwrap().position = (3+self.rotation_system.spawn_offsets[self.current_piece.as_ref().unwrap().id].0, 20+self.rotation_system.spawn_offsets[self.current_piece.as_ref().unwrap().id].1);
        match self.hold {
            Some(_) => {
                swap(&mut self.current_piece, &mut self.hold);
            }
            None => {
                self.hold = self.current_piece;
                self.from_next_to_current();
            },
        }
        self.can_hold = false;
        true
    }

    pub fn lock_current_piece(&mut self) -> bool {
        if self.position_is_valid((self.current_piece.as_ref().unwrap().position.0, self.current_piece.as_ref().unwrap().position.1-1), self.current_piece.as_ref().unwrap().rotation) {
            return false;
        }
        let minos_to_write = &self.rotation_system.pieces[self.current_piece.as_ref().unwrap().id][self.current_piece.as_ref().unwrap().rotation];
        for mino in minos_to_write{
            let x = (self.current_piece.as_ref().unwrap().position.0 + mino.0 as isize) as usize;
            let y = (self.current_piece.as_ref().unwrap().position.1 + mino.1 as isize) as usize;
            self.board.board[y][x] = Some(Mino{ color: self.rotation_system.colours[self.current_piece.as_ref().unwrap().id] });
        }
        self.current_piece = None;
        self.need_to_lock = false;
        return true;
    }

    pub fn sonic_drop(&mut self) -> bool {
        if self.current_piece.is_none(){
            return false;
        }
        if !self.position_is_valid((self.current_piece.as_ref().unwrap().position.0, self.current_piece.as_ref().unwrap().position.1-1), self.current_piece.as_ref().unwrap().rotation) {
            return false;
        }
        self.current_piece.as_mut().unwrap().position.1 = self.lowest_point_under_current_piece();
        true
    }

    pub fn lowest_point_under_current_piece(&self) -> isize{
        let mut y = self.current_piece.as_ref().unwrap().position.1;
        while self.position_is_valid((self.current_piece.as_ref().unwrap().position.0, y-1), self.current_piece.as_ref().unwrap().rotation){
            y -= 1
        }     
        y
    }

    fn reset_lock_delay(&mut self){
        match self.rotation_system.lock_delay_mode {
            LockDelayMode::Disabled => {},
            LockDelayMode::Gravity => {
                if self.position_is_valid((self.current_piece.as_ref().unwrap().position.0, self.current_piece.as_ref().unwrap().position.1-1), self.current_piece.as_ref().unwrap().rotation) {
                    self.lock_delay_active = false;
                }
            },
            LockDelayMode::ResetOnYChange => {},
            LockDelayMode::ResetOnMovementLimited => {
                if !self.position_is_valid((self.current_piece.as_ref().unwrap().position.0, self.current_piece.as_ref().unwrap().position.1-1), self.current_piece.as_ref().unwrap().rotation){
                    self.lock_delay_left = self.lock_delay;
                    if self.lock_delay_resets_left == 0{
                        self.need_to_lock = true;
                    }else{
                        self.lock_delay_resets_left -= 1;
                        self.lock_delay_active = false;
                    }
                }
            },
            LockDelayMode::ResetOnMovement => {
                if !self.position_is_valid((self.current_piece.as_ref().unwrap().position.0, self.current_piece.as_ref().unwrap().position.1-1), self.current_piece.as_ref().unwrap().rotation){
                    self.lock_delay_left = self.lock_delay;
                    self.lock_delay_active = false;
                }
            },
        }
    }

    pub fn rotate_current_piece(&mut self, rotation: i8) -> bool {
        if self.current_piece.is_none(){
            return false;
        }
        let future_rotation = (self.current_piece.as_ref().unwrap().rotation as i8 + rotation) as usize % self.rotation_system.pieces[self.current_piece.as_ref().unwrap().id].len();
        let id_for_kicks: usize = if rotation == 1 {
            0
        }else{
            1
        };
        self.reset_lock_delay();
        for test in &self.rotation_system.kicks[self.current_piece.as_ref().unwrap().id][self.current_piece.as_ref().unwrap().rotation][id_for_kicks]{
            let future_position = (self.current_piece.as_ref().unwrap().position.0 + test.0 as isize, self.current_piece.as_ref().unwrap().position.1 + test.1 as isize);
            if self.position_is_valid(future_position, future_rotation) {
                self.current_piece.as_mut().unwrap().rotation = future_rotation;
                self.current_piece.as_mut().unwrap().position = future_position;
                return true;
            }
        }
        false
    }

    pub fn move_current_piece(&mut self, shift: (i8, i8)) -> bool {
        if (shift.0 == 0 && shift.1 == 0) || self.current_piece.is_none(){
            return true;
        }
        let future_position = (
            self.current_piece.as_ref().unwrap().position.0 + shift.0 as isize, // future X
            self.current_piece.as_ref().unwrap().position.1 + shift.1 as isize  // future Y
        );
        if self.position_is_valid(future_position, self.current_piece.as_ref().unwrap().rotation) {
            if shift.0 != 0 {self.reset_lock_delay();}
            self.current_piece.as_mut().unwrap().position = future_position;
            true
        }else {
            false
        }
    }

    pub fn position_is_valid(&self, future_position: (isize, isize), future_rotation: usize) -> bool {
        for mino in &self.rotation_system.pieces[self.current_piece.as_ref().unwrap().id][future_rotation]{
            match self.board.board.get((future_position.1 + mino.1 as isize) as usize) {
                Some(line) => match line.get((future_position.0 + mino.0 as isize) as usize) {
                    Some(cell) => match cell {
                        Some(_) => return false,
                        None => continue,
                    },
                    None => return false,
                },
                None => return false,
            }
        }
        true
    }
}