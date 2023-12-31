use bevy::prelude::*;
use self::{systems::*, resources::Engine, ui::spawn_hud};

mod rotation_systems;
mod systems;
mod components;
mod resources;
mod ui;
pub mod randomizers;

pub struct UBSGEngine;

impl Plugin for UBSGEngine{
    fn build(&self, app: &mut App) {
        app.init_resource::<Engine>().
            add_state::<GameStates>().
            add_state::<GameloopStates>().
            insert_resource(Engine::default()).
            //add_systems(Startup, init_engine).
            add_systems(Startup, spawn_hud).
            add_systems(OnEnter(GameloopStates::Init), init_engine).
            add_systems(Update, receive_input.run_if(in_state(GameStates::Gameplay))).
            add_systems(Update, das_and_arr.run_if(in_state(GameStates::Gameplay))).
            add_systems(FixedUpdate, gameloop.run_if(in_state(GameStates::Gameplay)).run_if(in_state(GameloopStates::Falling))).
            add_systems(FixedUpdate, run_spawn_delay.run_if(in_state(GameStates::Gameplay)).run_if(in_state(GameloopStates::Spawn))).
            add_systems(OnEnter(GameloopStates::AfterLocking), after_locking_routine).
            add_systems(OnEnter(GameloopStates::Falling), draw_next).
            add_systems(OnExit(GameloopStates::Spawn), spawn_routine).
            add_systems(Update, draw_board.run_if(in_state(GameStates::Gameplay))).
            add_systems(Update, receive_input_on_game_over.run_if(in_state(GameStates::GameOver)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameStates{
    #[default]
    Init,
    Gameplay,
    Pause,
    GameOver
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameloopStates {
    #[default]
    Init,
    Spawn,
    Falling,
    AfterLocking
}