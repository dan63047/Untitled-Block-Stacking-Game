use bevy::prelude::*;

#[derive(Component)]
pub struct BoardVisual{}

#[derive(Component, Clone, Copy)]
pub struct Mino{
    pub skin_index: usize
}