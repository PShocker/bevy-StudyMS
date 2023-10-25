//! Displays a single [`Sprite`], created from an image.

use animationsprite::{animation, AnimationSprite};
use background::{background, BackGround};
use bevy::{prelude::*, window::WindowMode};
use std::{
    cmp::{max, min},
    fs,
};
use utils::composite_zindex;

use crate::utils::{cal_ax, cal_ay};
mod animationsprite;
mod background;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1600.0, 960.0).into(),
                title: "StudyMS".into(),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, animation)
        // .add_systems(Update, background)
        .run();
}

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let path = "./assets/Map/Map/Map0/000010000.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).unwrap();

    // print!("{:?}\n", p);

    commands.spawn(Camera2dBundle::default());

    let mut i = 0;
    for value in res["Layers"].as_array().unwrap() {
        // println!("{:?}", value);
        i += 1;
        if value["Objs"].as_array() != None {
            for objs in value["Objs"].as_array().unwrap() {
                let x = objs["X"].as_f64().unwrap() as f32;
                let y = -objs["Y"].as_f64().unwrap() as f32;
                let z = composite_zindex(
                    i,
                    objs["Z"].as_i64().unwrap() as i128,
                    objs["ID"].as_i64().unwrap() as i128,
                    0,
                ) as f32
                    / 1000000000.0;

                let mut animationsprite = AnimationSprite {
                    index: -1,
                    sprite: Vec::new(),
                    delays: Vec::new(),
                    start: false,
                    lastsprite: None,
                    delay: 0.0,
                };
                if objs["Resource"]["Frames"].as_array() != None {
                    for frames in objs["Resource"]["Frames"].as_array().unwrap() {
                        let ox = cal_ax(
                            frames["OriginX"].as_f64().unwrap() as f32,
                            frames["Width"].as_f64().unwrap() as f32,
                        );

                        let oy = -cal_ay(
                            frames["OriginY"].as_f64().unwrap() as f32,
                            frames["Height"].as_f64().unwrap() as f32,
                        );
                        println!("{:?}", frames["ResourceUrl"]);
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

                    commands.spawn(animationsprite);
                    // commands.
                }
            }
        }

        if value["Tiles"].as_array() != None {
            for tiles in value["Tiles"].as_array().unwrap() {
                // println!("{:?}", tiles);
                // println!("{:?}", tiles["Resource"]["ResourceUrl"]);
                let x = tiles["X"].as_f64().unwrap() as f32;
                let y = -tiles["Y"].as_f64().unwrap() as f32;
                // let z = tiles["ID"].as_f64().unwrap() as f32;
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
}
