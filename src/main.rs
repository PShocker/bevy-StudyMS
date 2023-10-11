//! Displays a single [`Sprite`], created from an image.

use bevy::{prelude::*, sprite::Anchor};
use std::fs;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920.0, 1080.0).into(),
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

    commands.spawn(Camera2dBundle::default());

    for value in res["Backs"].as_array().unwrap() {
        println!("{}", value["Resource"]["ResourceUrl"]);
        let x = value["X"].as_f64().unwrap() as f32;
        let y = value["Y"].as_f64().unwrap() as f32;
        let z = value["ID"].as_f64().unwrap() as f32;

        let ox = value["Resource"]["OriginX"].as_f64().unwrap() as f32/value["Resource"]["Width"].as_f64().unwrap() as f32/2.0;
        let oy = value["Resource"]["OriginY"].as_f64().unwrap() as f32/value["Resource"]["Height"].as_f64().unwrap() as f32/2.0;
        println!("{} and {}", x, y);
        commands.spawn(SpriteBundle {
            texture: asset_server.load(
                value["Resource"]["ResourceUrl"]
                    .to_string()
                    .replace("\"", ""),
            ),
            transform: Transform::from_xyz(x, y, z),
            sprite: Sprite {
                // anchor:bevy::sprite::Anchor::Custom(Vec2::new(ox,oy)),
                ..default()
            },
            // transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        });
    }
}
