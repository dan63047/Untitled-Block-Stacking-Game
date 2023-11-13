use bevy::prelude::*;

#[derive(Component)]
pub struct BoardVisual{}

#[derive(Component)]
pub struct HUD {}

#[derive(Component)]
pub struct LockDelayText {}

#[derive(Component, Clone, Copy)]
pub struct Mino{
    pub color: Color
}