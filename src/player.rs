use bevy::{prelude::*, render::render_phase::PhaseItem, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct Player;

pub fn player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("avatar.png"),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(13.0, 35.0),
        Velocity::zero(),
        Restitution::new(0.0),
        // GravityScale(5.0),
        Player,
    ));
}

// 角色奔跑
pub fn player_run(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<&mut Velocity, With<Player>>,
) {
    if q_player.is_empty() {
        return;
    }
    let mut velocity = q_player.single_mut();
    if keyboard_input.pressed(KeyCode::A) {
        velocity.linvel.x = -90.0;
    } else if keyboard_input.pressed(KeyCode::D) {
        velocity.linvel.x = 90.0;
    } else {
        // 不按键时停止左右奔跑
        velocity.linvel.x = 0.0;
    }
}
