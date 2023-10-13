//! Displays a single [`Sprite`], created from an image.

use bevy::prelude::*;
use std::{
    cmp::{max, min},
    fs,
};

fn composite_zindex(z0: i32, z1: i32, z2: i32) -> i32 {
    let scale = 1 << 10; // 1024
    let normalize = |mut v: i32| -> i32 {
        // -512 <= v <= 511
        v = v + scale / 2;
        // 0 <= v <= 1023
        v = max(0, min(v, scale - 1));
        return v;
    };
    return normalize(z0) * scale * scale + normalize(z1) * scale + normalize(z2);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (2000.0, 2000.0).into(),
                title: "StudyMS".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let path = "./assets/Map/Map/Map0/000010000.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).unwrap();

    // print!("{:?}\n", p);

    commands.spawn(Camera2dBundle::default());

    for value in res["Layers"].as_array().unwrap() {
        // println!("{:?}", value);
        if value["Tiles"].as_array() != None {
            for tiles in value["Tiles"].as_array().unwrap() {
                // println!("{:?}", tiles);
                println!("{:?}", tiles["Resource"]["ResourceUrl"]);
                let x = tiles["X"].as_f64().unwrap() as f32;
                let y = -tiles["Y"].as_f64().unwrap() as f32 + 320.0;
                // let z = tiles["ID"].as_f64().unwrap() as f32;
                let z = composite_zindex(
                    tiles["Resource"]["Z"].as_i64().unwrap() as i32,
                    tiles["ID"].as_i64().unwrap() as i32,
                    0,
                ) as f32
                    / 100000000.0;

                // let ox = tiles["Resource"]["OriginX"].as_f64().unwrap() as f32;
                // let oy = tiles["Resource"]["OriginY"].as_f64().unwrap() as f32;

                let ox = (tiles["Resource"]["OriginX"].as_f64().unwrap() as f32
                    - tiles["Resource"]["Width"].as_f64().unwrap() as f32 / 2.0)
                    / (tiles["Resource"]["Width"].as_f64().unwrap() as f32);

                let oy = -(tiles["Resource"]["OriginY"].as_f64().unwrap() as f32
                - tiles["Resource"]["Height"].as_f64().unwrap() as f32 / 2.0)
                / (tiles["Resource"]["Height"].as_f64().unwrap() as f32);

                println!("{} and {} and {}", ox, oy, z);
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
}
