mod engine;

use bevy::prelude::*;
use engine::UBSGEngine;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins)
        .add_plugins(UBSGEngine)
        .add_systems(Startup, startup)
        //.add_systems(Update, gameloop)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}

fn startup(
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
}

// fn gameloop(
//     mut camera: Query<(Entity), With<Camera>>,
//     time: Res<Time>,
// ) {
    
// }

// //! Shows how to create systems that run every fixed timestep, rather than every tick.

// use bevy::prelude::*;

// const FIXED_TIMESTEP: f32 = 1.0/60.0;
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         // this system will run once every update (it should match your screen's refresh rate)
//         //.add_systems(Update, frame_update)
//         // add our system to the fixed timestep schedule
//         .add_systems(FixedUpdate, fixed_update)
//         // configure our fixed timestep schedule to run twice a second
//         .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
//         .run();
// }

// /// your mom
// fn frame_update(mut last_time: Local<f32>, time: Res<Time>) {
//     info!(
//         "time since last frame_update: {}",
//         time.raw_elapsed_seconds() - *last_time
//     );
//     *last_time = time.raw_elapsed_seconds();
// }

// fn fixed_update(mut last_time: Local<f32>, time: Res<Time>, fixed_time: Res<FixedTime>) {
//     info!(
//         "time since last fixed_update: {}",
//         time.raw_elapsed_seconds() - *last_time
//     );

//     info!("fixed timestep: {}", FIXED_TIMESTEP);
//     info!(
//         "FPS: {}\n",
//         fixed_time.accumulated().as_secs_f32()/FIXED_TIMESTEP*60.
//     );
//     *last_time = time.raw_elapsed_seconds();
// }