use super::{resources::Engine, rotation_systems::LockDelayMode, GameStates, GameloopStates, randomizers::*};
use crate::engine::components::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const MINO_SIZE: f32 = 20.0;

pub fn reset_engine(mut commands: Commands){
    commands.remove_resource::<Engine>();
    commands.insert_resource::<Engine>(Engine::default());
}

pub fn init_engine(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut engine: ResMut<Engine>,
    mut old_board: Query<Entity, With<BoardVisual>>,
    mut next_state: ResMut<NextState<GameloopStates>>,
    mut game_next_state: ResMut<NextState<GameStates>>,
) {
    for board in old_board.iter() {
        commands.entity(board).despawn();
    }
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2 {
                        x: engine.board.width as f32 * MINO_SIZE,
                        y: engine.board.height as f32 * MINO_SIZE,
                    },
                    flip: false,
                }))
                .into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
        BoardVisual {},
    ));
    engine.init("NRS", Box::new(RandomWithoutDirectRepetition::create()));
    game_next_state.set(GameStates::Gameplay);
    next_state.set(GameloopStates::Falling);
}

pub fn draw_board(
    mut commands: Commands,
    engine: Res<Engine>,
    all_minos: Query<Entity, With<Mino>>,
    asset_server: Res<AssetServer>,
) {
    for mino in all_minos.iter() {
        commands.entity(mino).despawn();
    }
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    // draw board
    for row in &engine.board.board {
        for mino in row {
            match mino {
                Some(mino) => {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                x * MINO_SIZE - (engine.board.width as f32) / 2.0 * MINO_SIZE
                                    + MINO_SIZE / 2.0,
                                y * MINO_SIZE - (engine.board.height as f32) / 2.0 * MINO_SIZE
                                    + MINO_SIZE / 2.0,
                                0.0,
                            ),
                            texture: asset_server.load("default_mino.png"),
                            sprite: Sprite {
                                color: mino.color,
                                custom_size: Some(Vec2 {
                                    x: MINO_SIZE,
                                    y: MINO_SIZE,
                                }),
                                ..default()
                            },
                            ..default()
                        },
                        *mino,
                    ));
                }
                None => {}
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
            for mino in &engine.rotation_system.pieces[piece.id][piece.rotation] {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            x * MINO_SIZE - (engine.board.width as f32) / 2.0 * MINO_SIZE
                                + MINO_SIZE / 2.0
                                + mino.0 as f32 * MINO_SIZE,
                            y * MINO_SIZE - (engine.board.height as f32) / 2.0 * MINO_SIZE
                                + MINO_SIZE / 2.0
                                + mino.1 as f32 * MINO_SIZE,
                            1.0,
                        ),
                        texture: asset_server.load("default_mino.png"),
                        sprite: Sprite {
                            color: engine.rotation_system.colours[piece.id],
                            custom_size: Some(Vec2 {
                                x: MINO_SIZE,
                                y: MINO_SIZE,
                            }),
                            ..default()
                        },
                        ..default()
                    },
                    Mino {
                        color: engine.rotation_system.colours[piece.id],
                    },
                ));
            }
        }
        None => {}
    }

    // draw next queue
    if engine.board.show_next > 0 {
        y = 8.0;
        let mut drawed = 0;
        for mino in &engine.next_queue {
            for tile in &engine.rotation_system.pieces[mino.id]
                [mino.rotation]
            {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            11.0 * MINO_SIZE - (engine.board.width as f32) / 2.0 * MINO_SIZE
                                + MINO_SIZE / 2.0
                                + tile.0 as f32 * MINO_SIZE,
                            y * MINO_SIZE + MINO_SIZE / 2.0 + tile.1 as f32 * MINO_SIZE,
                            0.0,
                        ),
                        texture: asset_server.load("default_mino.png"),
                        sprite: Sprite {
                            color: engine.rotation_system.colours[mino.id],
                            custom_size: Some(Vec2 {
                                x: MINO_SIZE,
                                y: MINO_SIZE,
                            }),
                            ..default()
                        },
                        ..default()
                    },
                    Mino {
                        color: engine.rotation_system.colours[mino.id],
                    },
                ));
            }
            y -= 4.0;
            drawed += 1;
            if drawed >= engine.board.show_next {
                break;
            }
        }
    }

    // draw hold
    match engine.hold.as_ref() {
        Some(piece) => {
            for mino in &engine.rotation_system.pieces[piece.id][piece.rotation] {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            -5.0 * MINO_SIZE - (engine.board.width as f32) / 2.0 * MINO_SIZE
                                + MINO_SIZE / 2.0
                                + mino.0 as f32 * MINO_SIZE,
                            -2.0 * MINO_SIZE
                                + (engine.board.height as f32) / 2.0 * MINO_SIZE
                                + MINO_SIZE / 2.0
                                + mino.1 as f32 * MINO_SIZE,
                            0.0,
                        ),
                        texture: asset_server.load("default_mino.png"),
                        sprite: Sprite {
                            color: piece.color,
                            custom_size: Some(Vec2 {
                                x: MINO_SIZE,
                                y: MINO_SIZE,
                            }),
                            ..default()
                        },
                        ..default()
                    },
                    Mino { color: piece.color },
                ));
            }
        }
        None => {}
    }

    // draw shadow
    if engine.board.show_shadow {
        match engine.current_piece.as_ref() {
            Some(piece) => {
                x = piece.position.0 as f32;
                y = engine.lowest_point_under_current_piece() as f32;
                for mino in &engine.rotation_system.pieces[piece.id][piece.rotation] {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                x * MINO_SIZE - (engine.board.width as f32) / 2.0 * MINO_SIZE
                                    + MINO_SIZE / 2.0
                                    + mino.0 as f32 * MINO_SIZE,
                                y * MINO_SIZE - (engine.board.height as f32) / 2.0 * MINO_SIZE
                                    + MINO_SIZE / 2.0
                                    + mino.1 as f32 * MINO_SIZE,
                                0.0,
                            ),
                            texture: asset_server.load("default_mino.png"),
                            sprite: Sprite {
                                color: Color::Rgba {
                                    red: 1.0,
                                    green: 1.0,
                                    blue: 1.0,
                                    alpha: 0.1,
                                },
                                custom_size: Some(Vec2 {
                                    x: MINO_SIZE,
                                    y: MINO_SIZE,
                                }),
                                ..default()
                            },
                            ..default()
                        },
                        Mino {
                            color: engine.rotation_system.colours[piece.id],
                        },
                    ));
                }
            }
            None => {}
        }
    }
}

pub fn receive_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut engine: ResMut<Engine>,
    state: Res<State<GameloopStates>>,
    mut next_state: ResMut<NextState<GameloopStates>>,
    mut game_next_state: ResMut<NextState<GameStates>>,
    mut commands: Commands
) {
    if keyboard_input.just_pressed(KeyCode::R) && state.get() != &GameloopStates::Init {
        reset_engine(commands);
        next_state.set(GameloopStates::Init);
    }
    if keyboard_input.any_just_pressed([KeyCode::Up, KeyCode::X])
        && state.get() == &GameloopStates::Falling
    {
        engine.rotate_current_piece(1);
    }
    if keyboard_input.just_pressed(KeyCode::Z) && state.get() == &GameloopStates::Falling {
        engine.rotate_current_piece(-1);
    }
    if keyboard_input.just_pressed(KeyCode::C) && state.get() == &GameloopStates::Falling {
        engine.hold_current_piece();
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

pub fn receive_input_on_game_over(
    keyboard_input: Res<Input<KeyCode>>,
    state: Res<State<GameloopStates>>,
    mut next_state: ResMut<NextState<GameloopStates>>,
    mut commands: Commands
){
    if keyboard_input.just_pressed(KeyCode::R) && state.get() != &GameloopStates::Init {
        reset_engine(commands);
        next_state.set(GameloopStates::Init);
    } 
}

pub fn das_and_arr(mut engine: ResMut<Engine>, time: Res<Time>, state: Res<State<GameloopStates>>) {
    let direction = engine.handling.movement_tick(time.delta_seconds() * 1000.0);
    if state.get() == &GameloopStates::Falling {
        engine.move_current_piece((direction, 0));
    }
}

pub fn gameloop(
    clocks: Res<Time<Fixed>>,
    mut lock_delay_text: Query<&mut Text, With<LockDelayText>>,
    mut engine: ResMut<Engine>,
    mut next_state: ResMut<NextState<GameloopStates>>,
) {
    //info!("{:?}", clocks);
    match engine.current_piece {
        Some(piece) => {
            if engine.handling.sdf_active {
                engine.g_bucket += engine.g * engine.handling.sdf;
            } else {
                engine.g_bucket += engine.g;
            }
            let mut gravity_tick_happend = false;
            while engine.g_bucket >= 1.0 {
                engine.move_current_piece((0, -1));
                engine.g_bucket -= 1.0;
                gravity_tick_happend = true;
            }
            let previos_lock_delay_active = engine.lock_delay_active;
            engine.lock_delay_active =
                !engine.position_is_valid((piece.position.0, piece.position.1 - 1), piece.rotation);
            if engine.lock_delay_active {
                match engine.rotation_system.lock_delay_mode {
                    LockDelayMode::Disabled => {
                        engine.need_to_lock = true;
                    }
                    LockDelayMode::Gravity => {
                        if gravity_tick_happend && previos_lock_delay_active {
                            engine.need_to_lock = true;
                        }
                    }
                    LockDelayMode::ResetOnYChange => {
                        engine.lock_delay_left -= 1;
                    }
                    LockDelayMode::ResetOnMovementLimited => {
                        engine.lock_delay_left -= 1;
                    }
                    LockDelayMode::ResetOnMovement => {
                        engine.lock_delay_left -= 1;
                    }
                }
            } else {
                if previos_lock_delay_active {
                    match engine.rotation_system.lock_delay_mode {
                        LockDelayMode::Disabled => {}
                        LockDelayMode::Gravity => {}
                        LockDelayMode::ResetOnYChange => {
                            engine.lock_delay_left = engine.lock_delay;
                            if engine.lock_delay_resets == 0 {
                                engine.need_to_lock = true;
                            } else {
                                engine.lock_delay_resets_left -= 1;
                            }
                        }
                        LockDelayMode::ResetOnMovementLimited => {
                            engine.lock_delay_left = engine.lock_delay;
                        }
                        LockDelayMode::ResetOnMovement => {
                            engine.lock_delay_left = engine.lock_delay;
                        }
                    }
                }
            }
            if (engine.lock_delay_left < 1 || engine.need_to_lock)
                && !engine
                    .position_is_valid((piece.position.0, piece.position.1 - 1), piece.rotation)
            {
                engine.lock_current_piece();
                next_state.set(GameloopStates::AfterLocking);
            }
        }
        None => {}
    }
    for mut text in lock_delay_text.iter_mut() {
        text.sections[0].value = format!(
            "{}; {}",
            engine.lock_delay_resets_left, engine.lock_delay_left
        );
    }
}

pub fn after_locking_routine(
    mut engine: ResMut<Engine>,
    mut next_state: ResMut<NextState<GameloopStates>>,
) {
    engine.board.clear_full_lines();
    engine.lock_delay_left = engine.lock_delay;
    engine.lock_delay_resets_left = engine.lock_delay_resets;
    engine.lock_delay_active = false;
    next_state.set(GameloopStates::Spawn);
}

pub fn spawn_routine(
    mut engine: ResMut<Engine>,
    mut next_state: ResMut<NextState<GameloopStates>>,
    mut game_next_state: ResMut<NextState<GameStates>>,
) {
    if engine.spawn_sequence() {
        next_state.set(GameloopStates::Falling);
    } else {
        game_next_state.set(GameStates::GameOver);
    }
}
