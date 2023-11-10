use crate::common::*;
use bevy::{prelude::*, render::render_phase::PhaseItem, window::PrimaryWindow};
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
}

// 脸朝向
#[derive(Debug, Component, Clone, Copy, Default, PartialEq, Eq)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

#[derive(Debug, Resource, Clone, Copy, Default, PartialEq, Eq, Reflect)]
#[reflect(Resource)]
pub enum PlayerState {
    #[default]
    Standing,
    Walking,
    Jumping,
}

// 角色是否在地面上
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerGrounded(pub bool);

#[derive(Debug, Resource)]
pub struct PlayerStateAnimate {
    pub walk: AnimationBundle,
    pub stand: AnimationBundle,
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
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        animation_bundle: stand.clone(),
        rigid_body: RigidBody::Dynamic,
        rotation_constraints: LockedAxes::ROTATION_LOCKED,
        // Collider::cuboid(13.0, 32.0),
        collider: Collider::round_cuboid(7.0, 24.0, 0.1),
        velocity: Velocity::zero(),
        restitution: Restitution::new(0.0),
        gravity_scale: GravityScale(14.0),
        player: Player,
        facing: Facing::Right,
    });

    commands.insert_resource(PlayerStateAnimate {
        stand: stand,
        walk: walk,
    });
}

// 角色奔跑
pub fn player_run(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<
        (
            &mut Facing,
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut AnimationIndices,
            &mut AnimationTimer,
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
    for (mut facing, mut velocity, mut sprite, mut indices, mut timer) in &mut q_player {
        if keyboard_input.pressed(KeyCode::A) {
            if *player_state != PlayerState::Walking && player_grounded.0 {
                *player_state = PlayerState::Walking;
                *indices = player_ani.walk.indices.clone();
                *timer = player_ani.walk.timer.clone();
                state_change_ev.send_default(); //人物状态切换
            }
            *facing = Facing::Left;
            sprite.flip_x = false;
            velocity.linvel.x = -180.0;
        } else if keyboard_input.pressed(KeyCode::D) {
            if *player_state != PlayerState::Walking && player_grounded.0 {
                *player_state = PlayerState::Walking;
                *indices = player_ani.walk.indices.clone();
                *timer = player_ani.walk.timer.clone();
                state_change_ev.send_default(); //人物状态切换
            }
            *facing = Facing::Right;
            sprite.flip_x = true;
            velocity.linvel.x = 180.0;
        } else {
            if *player_state != PlayerState::Standing && player_grounded.0 {
                *player_state = PlayerState::Standing;
                *indices = player_ani.stand.indices.clone();
                *timer = player_ani.stand.timer.clone();
                state_change_ev.send_default(); //人物状态切换
                
            }
            velocity.linvel.x = 0.0;
        }

        if keyboard_input.pressed(KeyCode::AltLeft) {
            if player_grounded.0 {
                *player_state = PlayerState::Jumping;
                *indices = player_ani.walk.indices.clone();
                *timer = player_ani.walk.timer.clone();
                state_change_ev.send_default(); //人物状态切换
                velocity.linvel.y = 300.0;
            }
        }
    }
}

pub fn player_grounded_detect(
    q_player: Query<&Transform, With<Player>>,
    mut player_grounded: ResMut<PlayerGrounded>,
    mut last: Local<(f32, isize)>,
    mut player_state: ResMut<PlayerState>,
) {
    if q_player.is_empty() {
        return;
    }
    // 通过检测y轴坐标连续多帧是否变化来判断落地
    let pos = q_player.single().translation.truncate();
    if (pos.y * 10.).round() == last.0 {
        last.1 += 1;
    } else {
        last.1 -= 1;
    }
    last.1 = last.1.clamp(0, 5);

    if last.1 == 5 && !player_grounded.0 {
        //接触到地面
        player_grounded.0 = true;
    } else if last.1 < 2 && player_grounded.0 {
        //在空中
        player_grounded.0 = false;
    }

    last.0 = (pos.y * 10.).round();
}

pub fn setup_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut walk: Vec<Handle<Image>> = Vec::new();
    walk.push(asset_server.load("walk0.png"));
    walk.push(asset_server.load("walk1.png"));
    walk.push(asset_server.load("walk2.png"));
    walk.push(asset_server.load("walk3.png"));

    let mut stand: Vec<Handle<Image>> = Vec::new();
    stand.push(asset_server.load("stand0.png"));
    stand.push(asset_server.load("stand1.png"));
    stand.push(asset_server.load("stand2.png"));

    commands.insert_resource(PlayerAssets {
        stand: stand,
        walk: walk,
    });
}
