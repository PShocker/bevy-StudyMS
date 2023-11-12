use crate::{
    animate::{AnimationBundle, AnimationIndices, AnimationTimer},
    state_machine::*,
};
use bevy::{prelude::*, render::render_phase::PhaseItem, window::PrimaryWindow, transform::commands};
use bevy_rapier2d::prelude::*;

// 人物状态切换
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Event)]
pub struct StateChangeEvent;

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Player;

#[derive(Debug, Resource)]
pub struct PlayerAssets {
    pub walk: Vec<Handle<Image>>,
    pub stand: Vec<Handle<Image>>,
    pub jump: Vec<Handle<Image>>,
    pub prone: Vec<Handle<Image>>,
}

// 脸朝向
#[derive(Debug, Component, Clone, Copy, Default, PartialEq, Eq)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

#[derive(Debug, Resource, Clone, Copy, Default, PartialEq, Eq, Reflect, Component)]
#[reflect(Resource)]
pub enum PlayerState {
    #[default]
    Standing,
    Walking,
    Jumping,
    Prone,
}

// 角色是否在地面上
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerGrounded{
    pub flag:bool,
    pub enity:Option<Entity>,
}


#[derive(Debug, Resource)]
pub struct PlayerStateAnimate {
    pub walk: AnimationBundle,
    pub stand: AnimationBundle,
    pub jump: AnimationBundle,
    pub prone: AnimationBundle,
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
    pub state: PlayerState,
}

pub fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    assets: Res<PlayerAssets>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &assets.stand {
        let Some(texture) = textures.get(&handle) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                asset_server.get_handle_path(handle)
            );
            continue;
        };
        texture_atlas_builder.add_texture(handle.clone(), texture);
    }

    for handle in &assets.walk {
        let Some(texture) = textures.get(&handle) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                asset_server.get_handle_path(handle)
            );
            continue;
        };
        texture_atlas_builder.add_texture(handle.clone(), texture);
    }

    for handle in &assets.jump {
        let Some(texture) = textures.get(&handle) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                asset_server.get_handle_path(handle)
            );
            continue;
        };
        texture_atlas_builder.add_texture(handle.clone(), texture);
    }

    for handle in &assets.prone {
        let Some(texture) = textures.get(&handle) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                asset_server.get_handle_path(handle)
            );
            continue;
        };
        texture_atlas_builder.add_texture(handle.clone(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    let mut stand_indices = Vec::new();
    for handle in &assets.stand {
        stand_indices.push(texture_atlas.get_texture_index(handle).unwrap())
    }
    let stand = AnimationBundle {
        timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        indices: AnimationIndices {
            index: 0,
            sprite_indices: stand_indices,
        },
    };

    let mut walk_indices = Vec::new();
    for handle in &assets.walk {
        walk_indices.push(texture_atlas.get_texture_index(handle).unwrap())
    }
    let walk = AnimationBundle {
        timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        indices: AnimationIndices {
            index: 0,
            sprite_indices: walk_indices,
        },
    };

    let mut jump_indices = Vec::new();
    for handle in &assets.jump {
        jump_indices.push(texture_atlas.get_texture_index(handle).unwrap())
    }
    let jump = AnimationBundle {
        timer: AnimationTimer(Timer::from_seconds(0.0, TimerMode::Repeating)),
        indices: AnimationIndices {
            index: 0,
            sprite_indices: jump_indices,
        },
    };

    let mut prone_indices = Vec::new();
    for handle in &assets.prone {
        prone_indices.push(texture_atlas.get_texture_index(handle).unwrap())
    }
    let prone = AnimationBundle {
        timer: AnimationTimer(Timer::from_seconds(0.0, TimerMode::Repeating)),
        indices: AnimationIndices {
            index: 0,
            sprite_indices: prone_indices,
        },
    };

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        PlayerBundle {
            sprite_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite{
                    index: 0,
                    anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, -0.4)),
                    ..default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                ..default()
            },
            animation_bundle: stand.clone(),
            rigid_body: RigidBody::Dynamic,
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            // collider:Collider::ball(8.0),
            collider: Collider::round_cuboid(0.8, 0.8, 0.11),
            velocity: Velocity::zero(),
            restitution: Restitution::new(0.0),
            gravity_scale: GravityScale(16.0),
            player: Player,
            facing: Facing::Right,
            state: PlayerState::Standing,
        },
        ActiveEvents::CONTACT_FORCE_EVENTS,
    ));

    commands.insert_resource(PlayerStateAnimate {
        stand: stand,
        walk: walk,
        jump: jump,
        prone:prone,
    });
}

// 角色奔跑
pub fn player_run(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut q_player: Query<
        (
            &mut Facing,
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut AnimationIndices,
            &mut AnimationTimer,
            &mut Transform,
        ),
        With<Player>,
    >,
    mut player_state: ResMut<PlayerState>,
    player_ani: Res<PlayerStateAnimate>,
    mut state_change_ev: EventWriter<StateChangeEvent>,
    mut player_grounded: ResMut<PlayerGrounded>,
) {
    if q_player.is_empty() {
        return;
    }

    for (mut facing, mut velocity, mut sprite, mut indices, mut timer, mut transform) in
        &mut q_player
    {
        if keyboard_input.pressed(KeyCode::Left) {
            if player_grounded.flag {
                velocity.linvel.x = -180.0;
            }
            *facing = Facing::Left;
            sprite.flip_x = false;
        } else if keyboard_input.pressed(KeyCode::Right) {
            if player_grounded.flag {
                velocity.linvel.x = 180.0;
            }
            *facing = Facing::Right;
            sprite.flip_x = true;
        } else {
            if player_grounded.flag {
                velocity.linvel.x = 0.0;
            }
        }

        if keyboard_input.pressed(KeyCode::Down) {
            if keyboard_input.pressed(KeyCode::AltLeft) && player_grounded.flag {
                // transform.translation.y -= 50.0;
                //下跳
                if !commands.get_entity(player_grounded.enity.unwrap()).is_none() {
                    // commands.entity(player_grounded.enity.unwrap()).despawn();
                    commands.entity(player_grounded.enity.unwrap()).remove::<Collider>();
                }
                
            }else if player_grounded.flag{
                *player_state=PlayerState::Prone;
                return;
            }
        } else if keyboard_input.pressed(KeyCode::AltLeft) {
            if player_grounded.flag {
                velocity.linvel.y = 500.0;
            }
        }
        *player_state=PlayerState::Standing;
    }
}

//通过碰撞检测人物是否在地面上
pub fn player_grounded_detect(
    mut player_grounded: ResMut<PlayerGrounded>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut player_state: ResMut<PlayerState>,
    mut q_player: Query<
        (
            &mut TextureAtlasSprite,
            &mut AnimationIndices,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
    player_ani: Res<PlayerStateAnimate>,
    mut state_change_ev: EventWriter<StateChangeEvent>,
) {
    // for contact_force_event in contact_force_events.iter() {
    //     println!("Received contact force event: {contact_force_event:?}");
    // }
    let event=contact_force_events.iter().next();
    if event.is_some() {
        player_grounded.flag = true;
        // event.unwrap().collider
        player_grounded.enity = Some(event.unwrap().collider1);

        // println!("Received contact force event: {event:?}");
    } else {
        player_grounded.flag = false;
    }
}

pub fn setup_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut prone: Vec<Handle<Image>> = Vec::new();
    prone.push(asset_server.load("prone0.png"));


    let mut walk: Vec<Handle<Image>> = Vec::new();
    walk.push(asset_server.load("walk0.png"));
    walk.push(asset_server.load("walk1.png"));
    walk.push(asset_server.load("walk2.png"));
    walk.push(asset_server.load("walk3.png"));

    let mut stand: Vec<Handle<Image>> = Vec::new();
    stand.push(asset_server.load("stand0.png"));
    stand.push(asset_server.load("stand1.png"));
    stand.push(asset_server.load("stand2.png"));

    let mut jump: Vec<Handle<Image>> = Vec::new();
    jump.push(asset_server.load("jump0.png"));


    commands.insert_resource(PlayerAssets {
        stand: stand,
        walk: walk,
        jump: jump,
        prone:prone,
    });
}
