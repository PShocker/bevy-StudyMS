use crate::{
    animate::{AnimationBundle, AnimationIndices, AnimationTimer},
    customfilter::CustomFilterTag,
    state_machine::*,
    AppState,
};
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

// 人物状态切换
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Event)]
pub struct StateChangeEvent;

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Player;

#[derive(Debug, Resource)]
pub struct PlayerAssets {
    pub map:HashMap<String,Vec<Handle<Image>>>,
    // pub walk: Vec<Handle<Image>>,
    // pub stand: Vec<Handle<Image>>,
    // pub jump: Vec<Handle<Image>>,
    // pub prone: Vec<Handle<Image>>,
}

// 脸朝向
#[derive(Debug, Component, Clone, Copy, Default, PartialEq, Eq)]
pub enum Direction {
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
pub struct PlayerGrounded {
    pub flag: bool,
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
    pub facing: Direction,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub rotation_constraints: LockedAxes,
    pub velocity: Velocity,
    pub state: PlayerState,
    pub sleep: Sleeping,
    pub controller: KinematicCharacterController,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum LoadState {
    #[default]
    Setup,
    AssetsLoaded,
    PlayerFinished,
}


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::TextFinished), player) //生成人物
            .add_systems(
                Update,
                check_textures.run_if(in_state(AppState::SetupFinished)),//等待人物读取完成
            )
            .add_systems(OnEnter(AppState::Setup), setup_player_assets)
            .add_systems(
                Update,
                player_run.run_if(in_state(AppState::PlayerFinished)),
            ) //先读取人物动画,否则会导致读取失败
            .insert_resource(PlayerState::Standing)
            .insert_resource(PlayerGrounded { flag: false });
    }
}

//等待人物动作加载完成
fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    assets: ResMut<PlayerAssets>,
    image: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for map in assets.map {
        println!("{:?}", asset_server.get_group_load_state(map.1.iter().map(|h| h.id())));
    }
        // asset_server.get_group_load_state(assets.walk.iter().map(|h| h.id()));

        // next_state.set(AppState::TextFinished);
}

pub fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    assets: Res<PlayerAssets>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for map in assets.map{
        for vecs in map.1{
            let Some(texture) = textures.get(&vecs) else {
                warn!(
                    "{:?} did not resolve to an `Image` asset.",
                    asset_server.get_handle_path(vecs)
                );
                continue;
            };
            texture_atlas_builder.add_texture(vecs.clone(), texture);
        } 
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
                sprite: TextureAtlasSprite {
                    index: 0,
                    anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, -0.4)),
                    ..default()
                },
                // texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                ..default()
            },
            animation_bundle: stand.clone(),
            rigid_body: RigidBody::KinematicPositionBased,
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            collider: Collider::cuboid(9.0, 4.0),
            velocity: Velocity::zero(),
            restitution: Restitution::new(0.0),
            player: Player,
            facing: Direction::Right,
            state: PlayerState::Standing,
            sleep: Sleeping::disabled(),
            controller:KinematicCharacterController::default()
        },
        CustomFilterTag::GroupA,
    ));

    commands.insert_resource(PlayerStateAnimate {
        stand: stand,
        walk: walk,
        jump: jump,
        prone: prone,
    });
    next_state.set(AppState::PlayerFinished);
}

pub fn player_run(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut q_char: Query<&mut KinematicCharacterController>,
    mut q_out: Query<&mut KinematicCharacterControllerOutput>,
) {
    let mut player = q_char.single_mut();
    for (output) in q_out.iter() {
        if output.grounded == true {
            println!("touches the ground: {:?}", output.grounded);
        } else {
            println!("touches the ground: {:?}", output.grounded);
        }
    }

    let mut translation = Vec2::new(0.0, 0.0);

    if input.pressed(KeyCode::Right) {
        translation.x += time.delta_seconds() * 200.0;
    }

    if input.pressed(KeyCode::Left) {
        translation.x += time.delta_seconds() * 200.0 * -1.0;
    }

    translation.y += time.delta_seconds() * 200.0 * -1.0;

    player.translation = Some(translation);
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

    let mut map = HashMap::new();
    map.insert("prone", prone);
    map.insert("walk", walk);
    map.insert("stand", stand);
    map.insert("jump", jump);

    commands.insert_resource(PlayerAssets {
        map: map
    });
}
