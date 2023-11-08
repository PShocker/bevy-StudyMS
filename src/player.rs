use bevy::{prelude::*, render::render_phase::PhaseItem, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use crate::common::{ *};

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Player;

#[derive(Debug, Resource)]
pub struct PlayerAssets {
    pub assets: Vec<Handle<Image>>,
}

// 脸朝向
#[derive(Debug, Component, Clone, Copy, Default, PartialEq, Eq)]
pub enum Facing {
    Left,
    #[default]
    Right,
}



#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_bundle: SpriteSheetBundle,
    pub animation_bundle: AnimationBundle,
    pub facing: Facing,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub rotation_constraints: LockedAxes,
    pub velocity: Velocity,
    pub gravity_scale: GravityScale,
}


pub fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    assets: Res<PlayerAssets>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &assets.assets{
        
        let Some(texture) = textures.get(&handle) else {
            warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
            continue;
        };
        texture_atlas_builder.add_texture(handle.clone(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(PlayerBundle{
        sprite_bundle:SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(2),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        animation_bundle: AnimationBundle {
            timer: AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            indices: AnimationIndices {
                index: 0,
                sprite_indices: vec![1, 2, 3, 4],
            },
        },
        rigid_body:RigidBody::Dynamic,
        rotation_constraints:LockedAxes::ROTATION_LOCKED,
        // Collider::cuboid(13.0, 32.0),
        collider:Collider::round_cuboid(7.0, 24.0, 0.1),
        velocity:Velocity::zero(),
        restitution:Restitution::new(0.0),
        gravity_scale:GravityScale(12.0),
        player:Player,
        facing: Facing::Right,
        });
}

// 角色奔跑
pub fn player_run(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<(&mut Facing,&mut Velocity,&mut TextureAtlasSprite), With<Player>>,
) {
    if q_player.is_empty() {
        return;
    }
    for (mut facing,mut velocity, mut sprite) in &mut q_player {
        if keyboard_input.pressed(KeyCode::A) {
            *facing = Facing::Left;
            velocity.linvel.x = -180.0;
            sprite.flip_x = false;
        } else if keyboard_input.pressed(KeyCode::D) {
            *facing = Facing::Right;
            velocity.linvel.x = 180.0;
            sprite.flip_x = true;
        } else {
            velocity.linvel.x = 0.0;
        }
    }    
}


pub fn setup_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut assets: Vec<Handle<Image>> = Vec::new();
    assets.push(asset_server.load("avatar0.png"));
    assets.push(asset_server.load("avatar1.png"));
    assets.push(asset_server.load("avatar2.png"));
    assets.push(asset_server.load("avatar3.png"));

    commands.insert_resource(PlayerAssets {
        assets
    });
}
