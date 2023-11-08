use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::engine::components::*;
use super::{resources::Engine, GameloopStates};

const MINO_SIZE: f32 = 20.0;

pub fn init_engine(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut engine: ResMut<Engine>,
    mut next_state: ResMut<NextState<GameloopStates>>,
){
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad{ size: Vec2 { x: engine.board.width as f32 * MINO_SIZE, y: engine.board.height as f32 * MINO_SIZE }, flip: false })).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
        BoardVisual{}
    ));
    engine.temporary_random();
    next_state.set(GameloopStates::Falling);
}

pub fn draw_board(
    mut commands: Commands,
    engine: Res<Engine>,
    all_minos: Query<Entity, With<Mino>>,
    asset_server: Res<AssetServer>
){
    for mino in all_minos.iter() {
        commands.entity(mino).despawn();
    }
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    // draw board
    for row in &engine.board.grid {
        for mino in row {
            match mino {
                Some(mino) => {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                x*MINO_SIZE-(engine.board.width as f32)/2.0*MINO_SIZE+MINO_SIZE/2.0, 
                                y*MINO_SIZE-(engine.board.height as f32)/2.0*MINO_SIZE+MINO_SIZE/2.0, 
                                0.0
                            ),
                            texture: asset_server.load("skin.png"),
                            sprite: Sprite { 
                                rect: Some(
                                    Rect{
                                        min: Vec2 { x: 0.0+(64.0*mino.skin_index as f32), y: 0.0 },
                                        max: Vec2 { x: 63.0+(64.0*mino.skin_index as f32), y: 63.0 },
                                    }
                                ),
                                custom_size: Some(Vec2 {x: MINO_SIZE, y: MINO_SIZE}),
                                ..default()
                            },
                            ..default()
                        },
                        *mino,
                    ));
                }
                None => {},
            };
            x += 1.0;
        }
        x = 0.0;
        y += 1.0;
    }

    //draw current piece
    match engine.current_piece.as_ref() {
        Some(piece) => {
            x = piece.position.0 as f32;
            y = piece.position.1 as f32;
            for mino in &engine.rotation_system.pieces[piece.id][piece.rotation]{
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            x*MINO_SIZE - (engine.board.width  as f32)/2.0*MINO_SIZE + MINO_SIZE/2.0 + mino.0 as f32 * MINO_SIZE, 
                            y*MINO_SIZE - (engine.board.height as f32)/2.0*MINO_SIZE + MINO_SIZE/2.0 + mino.1 as f32 * MINO_SIZE,
                            0.0
                        ),
                        texture: asset_server.load("skin.png"),
                        sprite: Sprite { 
                            rect: Some(
                                Rect{
                                    min: Vec2 { x: 00.0+(64.0*engine.rotation_system.skin_index[piece.id] as f32), y: 00.0 },
                                    max: Vec2 { x: 63.0+(64.0*engine.rotation_system.skin_index[piece.id] as f32), y: 63.0 },
                                }
                            ),
                            custom_size: Some(Vec2 {x: MINO_SIZE, y: MINO_SIZE}),
                            ..default()
                        },
                        ..default()
                    },
                    Mino{skin_index: engine.rotation_system.skin_index[piece.id]},
                ));
            }
        },
        None => {},
    }
}

pub fn receive_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut engine: ResMut<Engine>,
    state: Res<State<GameloopStates>>,
    mut next_state: ResMut<NextState<GameloopStates>>,
){
    if keyboard_input.any_just_pressed([KeyCode::Up, KeyCode::X]) && state.get() == &GameloopStates::Falling {
        engine.rotate_current_piece(1);
    }
    if keyboard_input.just_pressed(KeyCode::Z) && state.get() == &GameloopStates::Falling {
        engine.rotate_current_piece(-1);
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        if state.get() == &GameloopStates::Falling {
            engine.move_current_piece((-1, 0));
        }
        engine.handling.movement_key_pressed(true, false)
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        if state.get() == &GameloopStates::Falling {
            engine.move_current_piece((1, 0));
        }
        engine.handling.movement_key_pressed(false, true)
    }
    if keyboard_input.just_released(KeyCode::Left) {
        engine.handling.movement_key_released(true, false)
    }
    if keyboard_input.just_released(KeyCode::Right) {
        engine.handling.movement_key_released(false, true)
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        engine.handling.sdf_active = true;
    }
    if keyboard_input.just_released(KeyCode::Down) {
        engine.handling.sdf_active = false;
    }
    if keyboard_input.just_pressed(KeyCode::Space) && state.get() == &GameloopStates::Falling {
        engine.sonic_drop();
        engine.lock_current_piece();
        next_state.set(GameloopStates::AfterLocking);
    }
}

pub fn das_and_arr(
    mut engine: ResMut<Engine>,
    time: Res<Time>,
    state: Res<State<GameloopStates>>,
){
    let direction = engine.handling.movement_tick(time.delta_seconds()*1000.0);
    if state.get() == &GameloopStates::Falling{
        engine.move_current_piece((direction, 0));
    } 
}

pub fn gameloop(
    clocks: Res<Time<Fixed>>,
    mut engine: ResMut<Engine>,
    mut next_state: ResMut<NextState<GameloopStates>>,
) {
    info!("{:?}", clocks);
    match engine.current_piece {
        Some(piece) => {
            engine.g_bucket += engine.g;
            if engine.handling.sdf_active {engine.g_bucket += engine.g * engine.handling.sdf}
            while engine.g_bucket >= 1.0 {
                engine.move_current_piece((0, -1));
                engine.g_bucket -= 1.0;
            }
            if !engine.position_is_valid((piece.position.0, piece.position.1-1), piece.rotation){
                engine.lock_delay_left -= 1;
            }else{
                engine.lock_delay_left = engine.lock_delay;
            }
            if engine.lock_delay_left < 1 && !engine.position_is_valid((piece.position.0, piece.position.1-1), piece.rotation){
                engine.lock_current_piece();
                next_state.set(GameloopStates::AfterLocking);
            }
        },
        None => {
            
        },
    }
}

pub fn after_locking_routine(
    mut engine: ResMut<Engine>,
    mut next_state: ResMut<NextState<GameloopStates>>,
){
    engine.board.clear_full_lines();
    engine.lock_delay_left = engine.lock_delay;
    engine.lock_delay_resets_left = engine.lock_delay_resets;
    engine.temporary_random();
    next_state.set(GameloopStates::Falling);
}