use bevy::{prelude::*, utils::HashMap};
use rand::prelude::*;
use lazy_static::lazy_static;
use bevy::sprite::{Sprite, MaterialMesh2dBundle};

const MINO_SIZE: f32 = 20.0;

pub struct UBSGEngine;

impl Plugin for UBSGEngine{
    fn build(&self, app: &mut App) {
        app.init_resource::<Engine>().
            add_state::<GameStates>().
            add_state::<GameloopStates>().
            insert_resource(Engine::default()).
            add_systems(Startup, init_engine).
            add_systems(Update, (receive_input, das_and_arr).run_if(in_state(GameloopStates::Falling))).
            add_systems(FixedUpdate, gameloop).
            add_systems(OnEnter(GameloopStates::AfterLocking), after_locking_routine).
            add_systems(Update, draw_board);
    }
}

#[derive(Clone)]
enum LockDelayMode{
    Disabled,
    Gravity,
    ResetOnYChange,
    ResetOnMovementLimited,
    ResetOnMovement
}

#[derive(Clone)]
struct PiecesData {
    // X and Y from bottom left point (pieces[piece][rotation] = Vec of coords for Minos)
    pieces: Vec<Vec<Vec<(u8, u8)>>>,
    // X and Y shifts for pieces (kicks[piece][rotation before spin][direction of rotation] = Vec of points for tests)
    kicks: Vec<Vec<Vec<Vec<(i8, i8)>>>>,
    // Takes 64x64 sprite fragment with this index as Mino skin (skin_index[piece])
    skin_index: Vec<usize>,
    // If spawn position is fucked, it fixes it
    spawn_offsets: Vec<(isize, isize)>,
    lock_delay_mode: LockDelayMode
}

lazy_static!{
    static ref ROTATION_SYSTEMS: HashMap<String, PiecesData> = {
        let mut rs = HashMap::new();
        rs.insert(String::from("SRS"), PiecesData{
            pieces: vec![
                vec![ // Z
                    vec![(0, 2), (1, 2), (1, 1), (2, 1)],
                    vec![(2, 2), (2, 1), (1, 1), (1, 0)],
                    vec![(2, 0), (1, 0), (1, 1), (0, 1)],
                    vec![(0, 0), (0, 1), (1, 1), (1, 2)]
                ],
                vec![ // J
                    vec![(0, 2), (0, 1), (1, 1), (2, 1)],
                    vec![(2, 2), (1, 2), (1, 1), (1, 0)],
                    vec![(2, 0), (2, 1), (1, 1), (0, 1)],
                    vec![(0, 0), (1, 0), (1, 1), (1, 2)]
                ],
                vec![ // I
                    vec![(0, 2), (1, 2), (2, 2), (3, 2)],
                    vec![(2, 3), (2, 2), (2, 1), (2, 0)],
                    vec![(3, 1), (2, 1), (1, 1), (0, 1)],
                    vec![(1, 0), (1, 1), (1, 2), (1, 3)]
                ],
                vec![ // T
                    vec![(1, 2), (0, 1), (1, 1), (2, 1)],
                    vec![(2, 1), (1, 2), (1, 1), (1, 0)],
                    vec![(1, 0), (2, 1), (1, 1), (0, 1)],
                    vec![(0, 1), (1, 0), (1, 1), (1, 2)]
                ],
                vec![ // O
                    vec![(0, 0), (0, 1), (1, 1), (1, 0)],
                    vec![(0, 1), (1, 1), (1, 0), (0, 0)],
                    vec![(1, 1), (1, 0), (0, 0), (0, 1)],
                    vec![(1, 0), (0, 0), (0, 1), (1, 1)]
                ],
                vec![ // L
                    vec![(2, 2), (2, 1), (1, 1), (0, 1)],
                    vec![(2, 0), (1, 0), (1, 1), (1, 2)],
                    vec![(0, 0), (0, 1), (1, 1), (2, 1)],
                    vec![(0, 2), (1, 2), (1, 1), (1, 0)]
                ],
                vec![ // S
                    vec![(2, 2), (1, 2), (1, 1), (0, 1)],
                    vec![(2, 0), (2, 1), (1, 1), (1, 2)],
                    vec![(0, 0), (1, 0), (1, 1), (2, 1)],
                    vec![(0, 2), (0, 1), (1, 1), (1, 0)]
                ]
            ],
            kicks: vec![
                vec![ // Z
                    vec![
                        vec![( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)], // 0 -> 90
                        vec![( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)], // 0 -> 270
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)], // 90 -> 180
                        vec![( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)], // 90 -> 0
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)], // 180 -> 270
                        vec![( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)], // 180 -> 90
                        ], 
                    vec![
                        vec![( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)], // 270 -> 0
                        vec![( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)], // 270 -> 180
                        ] 
                ],
                vec![ // J
                    vec![
                        vec![( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)], // 0 -> 90
                        vec![( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)], // 0 -> 270
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)], // 90 -> 180
                        vec![( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)], // 90 -> 0
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)], // 180 -> 270
                        vec![( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)], // 180 -> 90
                        ], 
                    vec![
                        vec![( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)], // 270 -> 0
                        vec![( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)], // 270 -> 180
                        ] 
                ],
                vec![ // I
                    vec![
                        vec![( 0, 0),(-2, 0),( 1, 0),(-2,-1),( 1, 2)], // 0 -> 90
                        vec![( 0, 0),(-1, 0),( 2, 0),(-1, 2),( 2,-1)], // 0 -> 270
                        ],
                    vec![
                        vec![( 0, 0),(-1, 0),( 2, 0),(-1, 2),( 2,-1)], // 90 -> 180
                        vec![( 0, 0),( 2, 0),(-1, 0),( 2, 1),(-1,-2)], // 90 -> 0
                        ],
                    vec![
                        vec![( 0, 0),( 2, 0),(-1, 0),( 2, 1),(-1,-2)], // 180 -> 270
                        vec![( 0, 0),( 1, 0),(-2, 0),( 1,-2),(-2, 1)], // 180 -> 90
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),(-2, 0),( 1,-2),(-2, 1)], // 270 -> 0
                        vec![( 0, 0),(-2, 0),( 1, 0),(-2,-1),( 1, 2)], // 270 -> 180
                        ]
                ],
                vec![ // T
                    vec![
                        vec![( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)], // 0 -> 90
                        vec![( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)], // 0 -> 270
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)], // 90 -> 180
                        vec![( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)], // 90 -> 0
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)], // 180 -> 270
                        vec![( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)], // 180 -> 90
                        ], 
                    vec![
                        vec![( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)], // 270 -> 0
                        vec![( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)], // 270 -> 180
                        ] 
                ],
                vec![ // O
                    vec![
                        vec![( 0, 0)], // 0 -> 90
                        vec![( 0, 0)], // 0 -> 270
                        ],
                    vec![
                        vec![( 0, 0)], // 90 -> 180
                        vec![( 0, 0)], // 90 -> 0
                        ],
                    vec![
                        vec![( 0, 0)], // 180 -> 270
                        vec![( 0, 0)], // 180 -> 90
                        ], 
                    vec![
                        vec![( 0, 0)], // 270 -> 0
                        vec![( 0, 0)], // 270 -> 180
                        ] 
                ],
                vec![ // L
                    vec![
                        vec![( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)], // 0 -> 90
                        vec![( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)], // 0 -> 270
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)], // 90 -> 180
                        vec![( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)], // 90 -> 0
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)], // 180 -> 270
                        vec![( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)], // 180 -> 90
                        ], 
                    vec![
                        vec![( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)], // 270 -> 0
                        vec![( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)], // 270 -> 180
                        ] 
                ],
                vec![ // S
                    vec![
                        vec![( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)], // 0 -> 90
                        vec![( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)], // 0 -> 270
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)], // 90 -> 180
                        vec![( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)], // 90 -> 0
                        ],
                    vec![
                        vec![( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)], // 180 -> 270
                        vec![( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)], // 180 -> 90
                        ], 
                    vec![
                        vec![( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)], // 270 -> 0
                        vec![( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)], // 270 -> 180
                        ] 
                ],
            ],
            skin_index: vec![0, 1, 2, 3, 4, 5, 6],
            spawn_offsets: vec![
                (0,  0), // Z
                (0,  0), // J
                (0, -1), // I
                (0,  0), // T
                (1,  1), // O
                (0,  0), // L
                (0,  0)  // S
            ],
            lock_delay_mode: LockDelayMode::ResetOnMovementLimited
        });
        rs
    };
}

#[derive(Component, Clone, Copy)]
struct Mino{
    skin_index: usize
}

#[derive(Clone, Copy)]
struct Piece{
    id: usize,
    position: (isize, isize),
    rotation: usize
}

#[derive(Component)]
struct BoardVisual{}

struct Board{
    width: u8,
    height: u8,
    buffer_height: u8,
    show_grid: bool,
    show_shadow: bool,
    // X axis - from left to right; Y axis - from bottom to top (grid[y][x])
    grid: Vec<Vec<Option<Mino>>>
}

impl Board{
    fn create(width: u8, height: u8, buffer_height: u8, show_grid: bool, show_shadow: bool) -> Board {
        let grid: Vec<Vec<Option<Mino>>> = vec![vec![None; width as usize]; (height+buffer_height) as usize];
        Board { width: width, height: height, buffer_height: buffer_height, show_grid: show_grid, show_shadow: show_shadow, grid: grid }
    }

    fn clear_full_lines(&mut self) {
        let mut lines_cleared: usize = 0;
        for row in 0..self.grid.len(){
            if self.grid[row-lines_cleared].iter().all(|l| l.is_some()){
                self.grid.remove(row-lines_cleared);
                let empty_row: Vec<Option<Mino>> = vec![None; self.width as usize];
                self.grid.push(empty_row);
                lines_cleared += 1;
            }
        }
    }
}

struct Handling{
    das: f32, // ms
    arr: f32, // ms
    sdf: f32, // gravity*sdf = soft drop
    das_left: f32, // ms
    arr_left: f32, // ms
    sdf_active: bool,
    active_left: bool,
    active_right: bool,
    direction: i8 // -1 - left, 1 - right, 0 - none
}

impl Handling {
    fn create(das: f32, arr: f32, sdf: f32) -> Handling{
        Handling { das: das, arr: arr, sdf: sdf, das_left: das, arr_left: arr, sdf_active:false, active_left: false, active_right: false, direction: 0}
    }

    fn movement_key_pressed(&mut self, left: bool, right: bool){
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

    fn movement_key_released(&mut self, left: bool, right: bool){
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

    fn movement_tick(&mut self, delta: f32) -> i8 {
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
struct Engine {
    current_piece: Option<Piece>,
    board: Board,
    handling: Handling,
    rotation_system: PiecesData,
    next_queue: Vec<Piece>,
    hold: Option<Piece>,
    can_hold: bool,
    g: f32,
    g_bucket: f32,
    lock_delay: u8,
    lock_delay_left: u8,
    lock_delay_resets: u8,
    lock_delay_resets_left: u8
}

impl Default for Engine {
    fn default() -> Engine {
        Engine {
            current_piece: None,
            board: Board::create(10, 20, 20, true, true),
            handling: Handling::create(200.0, 33.0, 20.0),
            rotation_system: ROTATION_SYSTEMS["SRS"].clone(),
            next_queue: vec![],
            hold: None,
            can_hold: true,
            g: 1.0/60.0,
            g_bucket: 0.0,
            lock_delay: 30,
            lock_delay_left: 30,
            lock_delay_resets: 15,
            lock_delay_resets_left: 15
        }
    }
}

impl Engine {
    fn temporary_random(&mut self){
        let piece_id = random::<usize>() % self.rotation_system.pieces.len();
        let final_position = (3+self.rotation_system.spawn_offsets[piece_id].0, 20+self.rotation_system.spawn_offsets[piece_id].1);
        self.current_piece = Some(Piece { id: piece_id, position: final_position, rotation: 0 });
        if self.g >= 20.0 { self.current_piece.as_mut().unwrap().position.1 = self.lowest_point_under_current_piece() }
    }

    fn lock_current_piece(&mut self) -> bool {
        if self.position_is_valid((self.current_piece.as_ref().unwrap().position.0, self.current_piece.as_ref().unwrap().position.1-1), self.current_piece.as_ref().unwrap().rotation) {
            return false;
        }
        let minos_to_write = &self.rotation_system.pieces[self.current_piece.as_ref().unwrap().id][self.current_piece.as_ref().unwrap().rotation];
        let color_index = self.rotation_system.skin_index[self.current_piece.as_ref().unwrap().id];
        for mino in minos_to_write{
            let x = (self.current_piece.as_ref().unwrap().position.0 + mino.0 as isize) as usize;
            let y = (self.current_piece.as_ref().unwrap().position.1 + mino.1 as isize) as usize;
            self.board.grid[y][x] = Some(Mino{ skin_index: color_index });
        }
        self.current_piece = None;
        return true;
    }

    fn sonic_drop(&mut self) -> bool {
        if !self.position_is_valid((self.current_piece.as_ref().unwrap().position.0, self.current_piece.as_ref().unwrap().position.1-1), self.current_piece.as_ref().unwrap().rotation) {
            return false;
        }
        self.current_piece.as_mut().unwrap().position.1 = self.lowest_point_under_current_piece();
        true
    }

    fn lowest_point_under_current_piece(&self) -> isize{
        let mut y = self.current_piece.as_ref().unwrap().position.1;
        while self.position_is_valid((self.current_piece.as_ref().unwrap().position.0, y-1), self.current_piece.as_ref().unwrap().rotation){
            y -= 1
        }     
        y
    }

    fn rotate_current_piece(&mut self, rotation: i8) -> bool {
        let future_rotation = (self.current_piece.as_ref().unwrap().rotation as i8 + rotation) as usize % self.rotation_system.pieces[self.current_piece.as_ref().unwrap().id].len();
        let id_for_kicks: usize = if rotation == 1 {
            0
        }else{
            1
        };
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

    fn move_current_piece(&mut self, shift: (i8, i8)) -> bool {
        if (shift.0 == 0 && shift.1 == 0) || self.current_piece.is_none(){
            return true;
        } 
        let future_position = (
            self.current_piece.as_ref().unwrap().position.0 + shift.0 as isize, // future X
            self.current_piece.as_ref().unwrap().position.1 + shift.1 as isize  // future Y
        );
        if self.position_is_valid(future_position, self.current_piece.as_ref().unwrap().rotation) {
            self.current_piece.as_mut().unwrap().position = future_position;
            true
        }else {
            false
        }
    }

    fn position_is_valid(&self, future_position: (isize, isize), future_rotation: usize) -> bool {
        for mino in &self.rotation_system.pieces[self.current_piece.as_ref().unwrap().id][future_rotation]{
            match self.board.grid.get((future_position.1 + mino.1 as isize) as usize) {
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

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameStates{
    #[default]
    Init,
    Gameplay,
    Pause,
    GameOver
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameloopStates {
    #[default]
    Init,
    Spawn,
    Falling,
    AfterLocking
}

fn init_engine(
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

fn draw_board(
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

fn receive_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut engine: ResMut<Engine>,
    mut next_state: ResMut<NextState<GameloopStates>>,
){
    if keyboard_input.any_just_pressed([KeyCode::Up, KeyCode::X]) {
        engine.rotate_current_piece(1);
    }
    if keyboard_input.just_pressed(KeyCode::Z) {
        engine.rotate_current_piece(-1);
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        engine.move_current_piece((-1, 0));
        engine.handling.movement_key_pressed(true, false)
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        engine.move_current_piece((1, 0));
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
    if keyboard_input.just_pressed(KeyCode::Space) {
        engine.sonic_drop();
        engine.lock_current_piece();
        next_state.set(GameloopStates::AfterLocking);
    }
}

fn das_and_arr(
    mut engine: ResMut<Engine>,
    time: Res<Time>
){
    let direction = engine.handling.movement_tick(time.delta_seconds()*1000.0);
    engine.move_current_piece((direction, 0));
}

fn gameloop(
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

fn after_locking_routine(
    mut engine: ResMut<Engine>,
    mut next_state: ResMut<NextState<GameloopStates>>,
){
    engine.board.clear_full_lines();
    engine.lock_delay_left = engine.lock_delay;
    engine.lock_delay_resets_left = engine.lock_delay_resets;
    engine.temporary_random();
    next_state.set(GameloopStates::Falling);
}