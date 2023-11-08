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