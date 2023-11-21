use crate::animate::{Animation, AnimationIndices, AnimationTimer};
use bevy::{app::RunFixedUpdateLoop, asset::LoadState, prelude::*, utils::HashMap};
use bevy_rapier2d::{na::ComplexField, prelude::*};

// 人物状态切换
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Event)]
pub struct StateChangeEvent;

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Player {
    pub translation: Vect,
    pub grounded: bool,
}

#[derive(Debug, Resource)]
pub struct PlayerAssets {
    pub handle_map: HashMap<String, Vec<Handle<Image>>>,
}

#[derive(Debug, Resource)]
pub struct AnimateAssets {
    pub animate_map: HashMap<String, Animation>,
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

#[derive(Component, Clone, Default, Debug)]
pub struct DownJumpTimer(pub Timer);

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_bundle: SpriteSheetBundle,
    pub animation: Animation,
    pub direction: Direction,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub rotation_constraints: LockedAxes,
    pub velocity: Velocity,
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

#[derive(Component)]
struct Jump(f32, f32);

const PLAYER_VELOCITY_X: f32 = 350.0;
const GRAVITY: f32 = 10.0;
//人在地砖上对地砖的力
const GROUND_FORCE: f32 = 1.0e-10;

const MAX_JUMP_HEIGHT: f32 = 7.6;

#[derive(Debug, Component, Clone, Default)]
pub struct Ground;
#[derive(Debug, Component, Clone, Default)]
pub struct Rise;
#[derive(Debug, Component, Clone, Default)]
pub struct Fall;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Load>()
            .add_systems(OnEnter(Load::Setup), setup_player_assets)
            .add_systems(OnEnter(Load::AssetsLoaded), player) //生成人物
            .add_systems(
                Update,
                check_textures.run_if(in_state(Load::Loading)), //等待人物读取完成
            )
            .add_systems(
                PreUpdate,
                update_groud.run_if(in_state(Load::PlayerFinished)),
            )
            .add_systems(
                RunFixedUpdateLoop,
                update_player_animation.run_if(in_state(Load::PlayerFinished)),
            )
            .add_systems(
                Update,
                (
                    update_flip,
                    // update_group,
                    update_fall,
                    update_ground,
                    update_downjump,
                    update_input,
                    update_rise,
                    update_direction,
                )
                    .run_if(in_state(Load::PlayerFinished)), //先读取人物动画,否则会导致读取失败
            )
            .add_event::<StateChangeEvent>();
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
        } else {
            next_state.set(Load::Loading);
        }
    }
}

fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    assets: ResMut<PlayerAssets>,
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
        let animate = Animation {
            timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            indices: AnimationIndices {
                index: 0,
                sprite_indices: indices,
            },
            name: map.0.to_string(),
        };
        animate_map.insert(map.0.to_string(), animate);
    }
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        PlayerBundle {
            sprite_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    // anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, -0.5)),
                    ..default()
                },
                // texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                ..default()
            },
            animation: animate_map.get("walk").unwrap().clone(),
            rigid_body: RigidBody::KinematicPositionBased,
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            collider: Collider::cuboid(16.0, 32.0),
            // collider: Collider::capsule_y(18.0, 16.0),
            velocity: Velocity::zero(),
            restitution: Restitution::new(0.0),
            player: Player {
                translation: Vect::ZERO,
                grounded: false,
            },
            direction: Direction::Right,
            sleep: Sleeping::disabled(),
            controller: KinematicCharacterController {
                filter_groups: Some(CollisionGroups::new(Group::GROUP_1, Group::ALL)),
                snap_to_ground: Some(CharacterLength::Absolute(1.0)),
                ..default()
            },
        },
        Friction::coefficient(1.0),
        Ccd::enabled(),
        Fall,
    ));
    commands.insert_resource(AnimateAssets {
        animate_map: animate_map,
    });
    next_state.set(Load::PlayerFinished);
}

fn update_rise(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Player, &mut KinematicCharacterController), With<Rise>>,
) {
    if query.is_empty() {
        return;
    }
    let (entity, mut player, mut controller) = query.single_mut();
    let dt = time.delta_seconds();
    player.translation.y -= GRAVITY * dt * 2.0;
    controller.translation = Some(Vec2::new(player.translation.x, player.translation.y));
    if player.translation.y <= 0.0 {
        commands.entity(entity).insert(Fall);
        commands.entity(entity).remove::<Rise>();
    }
}

fn update_input(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Player,
            &mut Animation,
            &mut KinematicCharacterController,
        ),
        With<Ground>,
    >,
    assets: ResMut<AnimateAssets>,
    mut state_change_ev: EventWriter<StateChangeEvent>,
) {
    if query.is_empty() {
        return;
    }
    // let (mut enity, mut player, mut controller, output) = query.single_mut();
    let (entity, mut player, mut animation, mut controller) = query.single_mut();
    player.translation.x = 0.0;
    player.translation.y = 0.0;
    if input.pressed(KeyCode::AltLeft) && input.pressed(KeyCode::Down) {
        player.translation.y = 2.4;
        commands
            .entity(entity)
            .insert(DownJumpTimer(Timer::from_seconds(0.4, TimerMode::Once)));
        commands.entity(entity).remove::<Ground>();
        commands.entity(entity).insert(Rise);

        let dt = time.delta_seconds();
        player.translation.y -= GRAVITY * dt * 2.0;
        controller.translation = Some(Vec2::new(player.translation.x, player.translation.y));
    } else if input.pressed(KeyCode::AltLeft) {
        player.translation.y = MAX_JUMP_HEIGHT;
        if input.pressed(KeyCode::Right) {
            player.translation.x = time.delta_seconds() * PLAYER_VELOCITY_X;
        } else if input.pressed(KeyCode::Left) {
            player.translation.x = time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
        }
        commands.entity(entity).remove::<Ground>();
        commands.entity(entity).insert(Rise);

        let dt = time.delta_seconds();
        player.translation.y -= GRAVITY * dt * 2.0;
        controller.translation = Some(Vec2::new(player.translation.x, player.translation.y));
    } else if !input.pressed(KeyCode::AltLeft) {
        if input.pressed(KeyCode::Right) {
            player.translation.x = time.delta_seconds() * PLAYER_VELOCITY_X;
        } else if input.pressed(KeyCode::Left) {
            player.translation.x = time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
        }
        controller.translation = Some(Vec2::new(player.translation.x, -GROUND_FORCE));
    }
    // player.translation = Some(translation);
}

fn update_fall(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Player, &mut KinematicCharacterController), With<Fall>>,
) {
    if query.is_empty() {
        return;
    }

    // let (mut enity, mut player, mut controller, output) = query.single_mut();
    let (entity, mut player, mut controller) = query.single_mut();
    let dt = time.delta_seconds();
    player.translation.y -= GRAVITY * dt * 2.0;
    controller.translation = Some(Vec2::new(player.translation.x, player.translation.y));
}

fn update_player_animation(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Animation,
            &mut Velocity,
            &mut KinematicCharacterControllerOutput,
            &mut Player,
        ),
        With<Player>,
    >,
    assets: ResMut<AnimateAssets>,
    mut state_change_ev: EventWriter<StateChangeEvent>,
) {
    if query.is_empty() {
        return;
    }
    let (entity, mut animation, mut velocity, mut output, mut player) = query.single_mut();
    // println!("{}",velocity.linvel.x);
    if output.desired_translation.x.abs() > 0.0 && output.grounded {
        //walk状态
        if animation.name != "walk" {
            commands
                .entity(entity)
                .insert(assets.animate_map.get("walk").unwrap().clone());
            state_change_ev.send_default();
        }
        // println!("walk");
    } else if output.desired_translation.x.abs() == 0.0 && output.grounded {
        //stand状态或prone状态
        if input.pressed(KeyCode::Down) {
            if animation.name != "prone" {
                commands
                    .entity(entity)
                    .insert(assets.animate_map.get("prone").unwrap().clone());
                state_change_ev.send_default();
            }
        } else {
            if animation.name != "stand" {
                commands
                    .entity(entity)
                    .insert(assets.animate_map.get("stand").unwrap().clone());
                state_change_ev.send_default();
            }
        }
    } else {
        //jump状态
        // *animation = assets.animate_map.get("jump").unwrap().clone();
        // println!("{:?},{:?}", velocity.linvel,output.desired_translation.y.abs());
        if animation.name != "jump" {
            commands
                .entity(entity)
                .insert(assets.animate_map.get("jump").unwrap().clone());
            state_change_ev.send_default();
        }
    }
    // println!("{:?}", animation);
}

//处理下跳
fn update_downjump(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut DownJumpTimer,
            &mut KinematicCharacterController,
        ),
        With<Player>,
    >,
) {
    if query.is_empty() {
        return;
    }
    let (entity, mut timer, mut controller) = query.single_mut();
    controller.filter_groups = Some(CollisionGroups::new(Group::GROUP_5, Group::ALL));
    if timer.0.tick(time.delta()).just_finished() {
        controller.filter_groups = Some(CollisionGroups::new(Group::GROUP_1, Group::ALL));
        commands.entity(entity).remove::<DownJumpTimer>();
    }
}

fn update_group(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut KinematicCharacterController,
        &mut KinematicCharacterControllerOutput,
        &mut Velocity,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, output, velocity) = query.single_mut();

    if player.filter_groups.unwrap().memberships == Group::GROUP_5 {
        return;
    }
    // println!("{:?}", velocity.linvel);
    // if  output.collisions.len()>0{

    // println!("{:?}",player.translation);
    // println!("{:?}",output.collisions[0].character_translation);
    // }

    let mut group = CollisionGroups::new(Group::GROUP_1, Group::ALL);
    if output.desired_translation.y < 0.0 {
        group.memberships = Group::GROUP_1;
    }
    if output.desired_translation.y >= 0.0 {
        group.memberships = Group::GROUP_2;
    }

    if output.desired_translation.x >= 0.0 {
        group.memberships = group.memberships | Group::GROUP_3;
    }
    if output.desired_translation.x <= 0.0 {
        group.memberships = group.memberships | Group::GROUP_4;
    }

    // println!("{:?}", group.memberships);

    player.filter_groups = Some(group);
}

fn update_flip(mut query: Query<(&mut TextureAtlasSprite, &Direction)>) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, direction) = query.single_mut();

    match direction {
        Direction::Right => sprite.flip_x = true,
        Direction::Left => sprite.flip_x = false,
    }
}

fn update_print(mut query: Query<&mut KinematicCharacterController>) {
    if query.is_empty() {
        return;
    }

    let player = query.single_mut();
    if player.translation != None {
        // println!("{:?}", player.translation);
    }
}

fn update_direction(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
) {
    if query.is_empty() {
        return;
    }
    let entity = query.single();
    if input.pressed(KeyCode::Right) {
        commands.entity(entity).insert(Direction::Right);
    } else if input.pressed(KeyCode::Left) {
        commands.entity(entity).insert(Direction::Left);
    }
}

//通过碰撞检测人物是否在地面上
pub fn update_groud(
    mut commands: Commands,
    mut query: Query<(Entity, &mut KinematicCharacterControllerOutput, &mut Player), With<Fall>>,
) {
    if query.is_empty() {
        return;
    }
    let (mut entity, mut output, mut player) = query.single_mut();

    if output.grounded {
        commands.entity(entity).insert(Ground);
        commands.entity(entity).remove::<Fall>();
    } else {
        commands.entity(entity).remove::<Ground>();
    }
}

//检测人物是否走到fh边缘并下落
pub fn update_ground(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut KinematicCharacterControllerOutput, &mut Player), With<Ground>>,
) {
    if query.is_empty() {
        return;
    }
    let (mut entity, mut output, mut player) = query.single_mut();

    if !output.grounded {
        player.translation.y = 0.0;
        commands.entity(entity).remove::<Ground>();
        commands.entity(entity).insert(Rise);
    }
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
