use super::components::{HUD, LockDelayText};
use bevy::prelude::*;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            // Enemy Text
            parent.spawn((
                TextBundle {
                    style: Style {
                        // position_type: PositionType::Absolute,
                        // top: Val::Percent(50.0),
                        // left: Val::Percent(50.0),
                        ..default()
                    },
                    text: Text {
                        sections: vec![TextSection::new(
                            "0",
                            TextStyle {
                                font: asset_server.load("EurostileRound-Regular.ttf"),
                                font_size: 64.0,
                                color: Color::rgb(1.0, 1.0, 1.0),
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                LockDelayText {},
            ));
        })
        .id();
    hud_entity
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}