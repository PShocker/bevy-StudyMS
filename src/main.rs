//! Displays a single [`Sprite`], created from an image.

use animate::{AnimatePlugin, Animations};
use background::{BackGround, BackGroundEdge, BackGroundPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::*;
use foothold::{FootHold, FootHoldType};
use player::PlayerPlugin;
use std::{
    cmp::{max, min},
    fs,
};
use utils::composite_zindex;

use crate::utils::{cal_ax, cal_ay};
mod animate;
mod background;
mod camera;
mod foothold;
mod player;
mod utils;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                //设置窗口大小 1100*750
                primary_window: Some(Window {
                    title: "StudyMS".to_owned(),
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(), //显示碰撞线
        ))
        .add_plugins(PlayerPlugin) //人物
        .add_plugins(CameraPlugin) //镜头跟随
        .add_plugins(AnimatePlugin) //动画
        .add_plugins(BackGroundPlugin) //生成背景
        .add_systems(Startup, setup) //初始化
        //人物行走输入事件和人物方向
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
                );

                //具有动画效果的obj
                let mut animationsprite = Animations {
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
                    // commands.spawn(animationsprite);
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
                );
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

                // commands.spawn(SpriteBundle {
                //     texture: asset_server.load(
                //         tiles["Resource"]["ResourceUrl"]
                //             .to_string()
                //             .replace("\"", ""),
                //     ),
                //     transform: Transform::from_xyz(x, y, z),
                //     sprite: Sprite {
                //         anchor: bevy::sprite::Anchor::Custom(Vec2::new(ox, oy)),
                //         ..default()
                //     },
                //     ..default()
                // });
            }
        }
        i += 1;
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
                    // commands.spawn(background);
                }
                1 => {}
                _ => println!("Ani Other"),
            }
            // print!("{:?}", backs);
        }
    }
    //解析地图FootHold
    /*
    线段绘制方向决定了单边碰撞的方向
    水平线段如果是从左往右画，那么从上往下移动会发生碰撞，从下往上移动不会发生碰撞--GruopA
    水平线段如果是从右往左画，那么从下往上移动会发生碰撞，从上往下移动不会发生碰撞--GruopB
    垂直线段如果是从上往下画，那么从右往左移动会发生碰撞，从左往右移动不会发生碰撞--GruopC
    垂直线段如果是从下往上画，那么从左往右移动会发生碰撞，从右往左移动不会发生碰撞--GruopD
    所有斜线只有从上往下会发生碰撞

    当某条垂直线段往下延伸的其他线段都是垂直线段，且没有出现拐弯就突然中断时，那么这条线段是无效的墙，永远不会发生碰撞

    当角色处于地面的时候，只能与相同layer的线段发生碰撞
    当角色处于空中的时候，能与所有非垂直线段发生碰撞，但是只能与相同layer的垂直线段发生碰撞
     */
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
                layer: foothold["Layer"].as_i64().unwrap() as i32,
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
                CollisionGroups::new(
                    FootHold::get_foothold_layer(foothold.layer),
                    FootHold::get_foothold_group(
                        Vec2::new(foothold.x1 as f32, -foothold.y1 as f32),
                        Vec2::new(foothold.x2 as f32, -foothold.y2 as f32),
                    ),
                ),
                FootHold::get_foothold_type(
                    Vec2::new(foothold.x1 as f32, -foothold.y1 as f32),
                    Vec2::new(foothold.x2 as f32, -foothold.y2 as f32),
                ),
                RigidBody::Fixed,
                foothold,
            ));
        }
        //地图左边墙壁
        commands.spawn((
            Collider::segment(
                Vec2::new(left as f32, -10000.0),
                Vec2::new(left as f32, 10000.0),
            ),
            RigidBody::Fixed,
            FootHoldType::Vertical,
            CollisionGroups::new(Group::ALL, Group::ALL),
        ));
        //地图右边墙壁
        commands.spawn((
            Collider::segment(
                Vec2::new(right as f32, -10000.0),
                Vec2::new(right as f32, 10000.0),
            ),
            RigidBody::Fixed,
            FootHoldType::Vertical,
            CollisionGroups::new(Group::ALL, Group::ALL),
        ));
    }
}
