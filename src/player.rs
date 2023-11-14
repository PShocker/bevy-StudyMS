use crate::{
    animate::{AnimationBundle, AnimationIndices, AnimationTimer},
    customfilter::CustomFilterTag,
    state_machine::*,
    AppState,
};
use bevy::{asset::LoadState, prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

// 人物状态切换
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Event)]
pub struct StateChangeEvent;

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Player;

#[derive(Debug, Resource)]
pub struct PlayerAssets {
    pub handle_map: HashMap<String, Vec<Handle<Image>>>,
}

#[derive(Debug, Resource)]
pub struct AnimateAssets {
    pub animate_map: HashMap<String, AnimationBundle>,
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
enum Load {
    #[default]
    Setup,
    Loading,
    AssetsLoaded,
    PlayerFinished,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Load>()
            .add_systems(OnEnter(Load::AssetsLoaded), player) //生成人物
            .add_systems(
                Update,
                check_textures.run_if(in_state(Load::Loading)), //等待人物读取完成
            )
            .add_systems(OnEnter(Load::Setup), setup_player_assets)
            .add_systems(Update, player_run.run_if(in_state(Load::PlayerFinished))) //先读取人物动画,否则会导致读取失败
            .insert_resource(PlayerState::Standing)
            .insert_resource(PlayerGrounded { flag: false });
    }
}

//等待人物动作加载完成
fn check_textures(
    mut next_state: ResMut<NextState<Load>>,
    assets: ResMut<PlayerAssets>,
    image: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for map in &assets.handle_map {
        if LoadState::Loaded == asset_server.get_group_load_state(map.1.iter().map(|h| h.id())) {
            next_state.set(Load::AssetsLoaded);
        }
    }
}

fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut assets: ResMut<PlayerAssets>,
    mut next_state: ResMut<NextState<Load>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for map in &assets.handle_map {
        for vecs in map.1 {
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

    let mut animate_map = HashMap::new();

    for map in &assets.handle_map {
        let mut indices = Vec::new();
        for handle in map.1 {
            indices.push(texture_atlas.get_texture_index(&handle).unwrap())
        }
        let animate = AnimationBundle {
            timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            indices: AnimationIndices {
                index: 0,
                sprite_indices: indices,
            },
        };
        animate_map.insert(map.0.to_string(), animate);
    }
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        PlayerBundle {
            sprite_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, -0.5)),
                    ..default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                ..default()
            },
            animation_bundle: animate_map.get("walk").unwrap().clone(),
            rigid_body: RigidBody::KinematicPositionBased,
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            collider: Collider::cuboid(9.0, 4.0),
            velocity: Velocity::zero(),
            restitution: Restitution::new(0.0),
            player: Player,
            facing: Direction::Right,
            state: PlayerState::Standing,
            sleep: Sleeping::disabled(),
            controller: KinematicCharacterController::default(),
        },
        CustomFilterTag::GroupA,
    ));
    commands.insert_resource(AnimateAssets {
        animate_map: animate_map,
    });
    next_state.set(Load::PlayerFinished);
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

fn setup_player_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<Load>>,
) {
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

    let mut handle_map = HashMap::new();
    handle_map.insert("prone".to_string(), prone);
    handle_map.insert("walk".to_string(), walk);
    handle_map.insert("stand".to_string(), stand);
    handle_map.insert("jump".to_string(), jump);

    commands.insert_resource(PlayerAssets {
        handle_map: handle_map,
    });
    next_state.set(Load::Loading);
}
