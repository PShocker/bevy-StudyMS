//! Displays a single [`Sprite`], created from an image.

use animate::{animate_back, animate_player, AnimateObj};
use background::{background, BackGround, BackGroundEdge};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::*;
use foothold::FootHold;
use player::{
    player, player_grounded_detect, player_run, setup_player_assets, PlayerAssets, PlayerGrounded,
    PlayerState, StateChangeEvent,
};
use std::{
    cmp::{max, min},
    fs,
};
use utils::composite_zindex;

use crate::utils::{cal_ax, cal_ay};
mod animate;
mod background;
mod camera;
mod common;
mod foothold;
mod player;
mod utils;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {collision_event:?}");
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {contact_force_event:?}");
    }
}

//等待人物动作加载完成
fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    assets: ResMut<PlayerAssets>,
    image: ResMut<Assets<Image>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for handle in &assets.stand {
        let Some(texture) = image.get(&handle) else {
            continue;
        };
        // next_state.set(AppState::Finished);
    }
    for handle in &assets.walk {
        let Some(texture) = image.get(&handle) else {
            continue;
        };
        next_state.set(AppState::Finished);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_state::<AppState>()
        .add_systems(Startup, setup) //初始化
        .add_systems(OnEnter(AppState::Setup), setup_player_assets) //先读取人物动画,否则会导致读取失败
        .add_systems(Update, check_textures.run_if(in_state(AppState::Setup))) //等待人物读取完成
        .add_systems(OnEnter(AppState::Finished), player) //生成人物
        .insert_resource(PlayerState::Standing)
        .insert_resource(PlayerGrounded(false))
        .add_systems(Update, animate_back) //背景动画
        .add_systems(Update, camera_follow) //镜头跟随
        .add_systems(
            Update,
            (player_run, player_grounded_detect).run_if(in_state(AppState::Finished)),
        ) //人物行走输入事件和人物方向
        .add_systems(Update, background) //背景跟随
        .add_systems(Update, animate_player) //播放人物动画
        .add_systems(PostUpdate, display_events)
        .add_event::<StateChangeEvent>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //读取地图json,json文件参考 https://github.com/Kagamia/MapRenderWeb.git
    //参考https://www.bilibili.com/video/BV1ou4y1o7XZ/
    let path = "./assets/Map/Map/Map0/000010000.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).unwrap();

    commands.spawn(Camera2dBundle::default());

    //解析背景的json文件,从Layer开始
    let mut i = 0;
    for value in res["Layers"].as_array().unwrap() {
        // println!("{:?}", value);
        i += 1;
        //i相当于layer,越大的i会覆盖较小的i值的物体
        if value["Objs"].as_array() != None {
            for objs in value["Objs"].as_array().unwrap() {
                let x = objs["X"].as_f64().unwrap() as f32;
                let y = -objs["Y"].as_f64().unwrap() as f32;
                //根据 z,id,i计算z值
                let z = composite_zindex(
                    i,
                    objs["Z"].as_i64().unwrap() as i128,
                    objs["ID"].as_i64().unwrap() as i128,
                    0,
                ) as f32
                    / 1000000000.0; //除以1000000000保证精度

                //具有动画效果的obj
                let mut animationsprite = AnimateObj {
                    index: -1,
                    sprite: Vec::new(),
                    delays: Vec::new(),
                    start: false,
                    lastsprite: None,
                    delay: 0.0,
                };
                if objs["Resource"]["Frames"].as_array() != None {
                    for frames in objs["Resource"]["Frames"].as_array().unwrap() {
                        //计算物体原点坐标
                        let ox = cal_ax(
                            frames["OriginX"].as_f64().unwrap() as f32,
                            frames["Width"].as_f64().unwrap() as f32,
                        );

                        let oy = -cal_ay(
                            frames["OriginY"].as_f64().unwrap() as f32,
                            frames["Height"].as_f64().unwrap() as f32,
                        );
                        // println!("{:?}", frames["ResourceUrl"]);
                        let s = SpriteBundle {
                            texture: asset_server
                                .load(frames["ResourceUrl"].to_string().replace("\"", "")),
                            transform: Transform::from_xyz(x, y, z),
                            sprite: Sprite {
                                anchor: bevy::sprite::Anchor::Custom(Vec2::new(ox, oy)),
                                ..default()
                            },
                            ..default()
                        };
                        animationsprite.sprite.push(s);
                        animationsprite
                            .delays
                            .push(frames["Delay"].as_i64().unwrap() as f32);
                    }
                    //产生组件,animate_back处理动画效果
                    commands.spawn(animationsprite);
                    // commands.
                }
            }
        }
        //从地图json解析Tiles
        if value["Tiles"].as_array() != None {
            for tiles in value["Tiles"].as_array().unwrap() {
                // println!("{:?}", tiles);
                // println!("{:?}", tiles["Resource"]["ResourceUrl"]);
                let x = tiles["X"].as_f64().unwrap() as f32;
                let y = -tiles["Y"].as_f64().unwrap() as f32;
                let z = composite_zindex(
                    i,
                    tiles["Resource"]["Z"].as_i64().unwrap() as i128,
                    tiles["ID"].as_i64().unwrap() as i128,
                    0,
                ) as f32
                    / 1000000000.0;
                // let ox = tiles["Resource"]["OriginX"].as_f64().unwrap() as f32;
                // let oy = tiles["Resource"]["OriginY"].as_f64().unwrap() as f32;

                let ox = cal_ax(
                    tiles["Resource"]["OriginX"].as_f64().unwrap() as f32,
                    tiles["Resource"]["Width"].as_f64().unwrap() as f32,
                );

                let oy = -cal_ay(
                    tiles["Resource"]["OriginY"].as_f64().unwrap() as f32,
                    tiles["Resource"]["Height"].as_f64().unwrap() as f32,
                );

                // println!("{} and {} and {}", x, y, z);
                // println!("{} and {}", tiles["ID"].as_i64().unwrap(), z);
                commands.spawn(SpriteBundle {
                    texture: asset_server.load(
                        tiles["Resource"]["ResourceUrl"]
                            .to_string()
                            .replace("\"", ""),
                    ),
                    transform: Transform::from_xyz(x, y, z),
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::Custom(Vec2::new(ox, oy)),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
    //解析地图Backs
    for backs in res["Backs"].as_array().unwrap() {
        if backs["Resource"].as_object() != None {
            // let resource = backs["Resource"].as_object().unwrap();
            // println!("{:?}", resource);
            match backs["Ani"].as_i64().unwrap() {
                0 => {
                    //sprite
                    let id = backs["ID"].as_i64().unwrap() as i32;
                    let x = backs["X"].as_i64().unwrap() as i32;
                    let y = -backs["Y"].as_i64().unwrap() as i32;
                    let cx = backs["Cx"].as_i64().unwrap() as i32;
                    let cy = backs["Cy"].as_i64().unwrap() as i32;
                    let rx = backs["Rx"].as_i64().unwrap() as i32;
                    let ry = backs["Ry"].as_i64().unwrap() as i32;
                    let alpha = backs["Alpha"].as_i64().unwrap() as i32;
                    let flip_x = backs["FlipX"].as_bool().unwrap();
                    let front = backs["Front"].as_bool().unwrap();
                    let ani = backs["Ani"].as_i64().unwrap() as i32;
                    let types = backs["Type"].as_i64().unwrap() as i32;
                    let resource = backs["Resource"].to_string();
                    let background = BackGround::new(
                        id, x, y, cx, cy, rx, ry, alpha, flip_x, front, ani, types, resource,
                    );
                    commands.spawn(background);
                }
                1 => {}
                _ => println!("Ani Other"),
            }
            // print!("{:?}", backs);
        }
    }
    //解析地图FootHold
    if res["FootHold"].as_array() != None {
        let mut left = 0;
        let mut right = 0;
        for foothold in res["FootHold"].as_array().unwrap() {
            // println!("{:?}", foothold);
            let foothold = FootHold {
                x1: foothold["X1"].as_i64().unwrap() as i32,
                x2: foothold["X2"].as_i64().unwrap() as i32,
                y1: foothold["Y1"].as_i64().unwrap() as i32,
                y2: foothold["Y2"].as_i64().unwrap() as i32,
                prev: foothold["Prev"].as_i64().unwrap() as i32,
                next: foothold["Next"].as_i64().unwrap() as i32,
                piece: foothold["Piece"].as_i64().unwrap() as i32,
                id: foothold["ID"].as_i64().unwrap() as i32,
            };
            if left > min(foothold.x1, foothold.x2) {
                left = min(foothold.x1, foothold.x2)
            }
            if right < max(foothold.x1, foothold.x2) {
                right = max(foothold.x1, foothold.x2)
            }
            commands.insert_resource(BackGroundEdge {
                left: left as f32,
                right: right as f32,
            });
            // commands.spawn(foothold);
            //直接用bevy_rapier2d生成地砖,使其具有物理效果
            commands.spawn((
                Collider::segment(
                    Vec2::new(foothold.x1 as f32, -foothold.y1 as f32),
                    Vec2::new(foothold.x2 as f32, -foothold.y2 as f32),
                ),
                Friction::coefficient(1.0),
            )); //摩擦力
        }
        //地图左边墙壁
        commands.spawn((Collider::segment(
            Vec2::new(left as f32, -10000.0),
            Vec2::new(left as f32, 10000.0),
        ),));
        //地图右边墙壁
        commands.spawn((Collider::segment(
            Vec2::new(right as f32, -10000.0),
            Vec2::new(right as f32, 10000.0),
        ),));
    }
}
