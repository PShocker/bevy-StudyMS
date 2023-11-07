use bevy::{prelude::*, render::render_phase::PhaseItem, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct Player;

#[derive(Debug, Resource)]
pub struct PlayerAssets {
    pub assets: Vec<Handle<Image>>,
}

pub fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut assets: Vec<Handle<Image>> = Vec::new();
    assets.push(asset_server.load("avatar0.png"));
    assets.push(asset_server.load("avatar1.png"));
    assets.push(asset_server.load("avatar2.png"));
    assets.push(asset_server.load("avatar3.png"));

    println!("{:?}",textures.get(&asset_server.load("avatar0.png")));
    println!("{:?}",textures.get(&asset_server.load("avatar1.png")));
    println!("{:?}",textures.get(&asset_server.load("avatar2.png")));
    println!("{:?}",textures.get(&asset_server.load("avatar3.png")));
    // let mut texture_atlas_builder = TextureAtlasBuilder::default();
    // for handle in &assets{
        
    //     let Some(texture) = textures.get(&handle) else {
    //         warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
    //         continue;
    //     };
    //     texture_atlas_builder.add_texture(handle.clone(), texture);
    // }

    // let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // commands.spawn((
    //     SpriteSheetBundle {
    //         sprite: TextureAtlasSprite::new(0),
    //         texture_atlas: texture_atlas_handle.clone(),
    //         transform: Transform::from_xyz(0.0, 0.0, 100.0),
    //         ..default()
    //     },
    //     RigidBody::Dynamic,
    //     LockedAxes::ROTATION_LOCKED,
    //     // Collider::cuboid(13.0, 32.0),
    //     Collider::round_cuboid(7.0, 22.0, 0.1),
    //     Velocity::zero(),
    //     Restitution::new(0.0),
    //     GravityScale(12.0),
    //     Player,
    // ));
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
        velocity.linvel.x = -180.0;
    } else if keyboard_input.pressed(KeyCode::D) {
        velocity.linvel.x = 180.0;
    } else {
        velocity.linvel.x = 0.0;
    }
}
