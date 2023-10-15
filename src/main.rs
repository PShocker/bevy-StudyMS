//! Displays a single [`Sprite`], created from an image.

use bevy::{prelude::*, window::WindowMode};
use std::{
    cmp::{max, min},
    fs,
};

fn composite_zindex(z0: i64, z1: i64, z2: i64) -> i64 {
    let scale = 1 << 10; // 1024
    let normalize = |mut v: i64| -> i64 {
        // v = v.abs();
        v = v + scale / 2;
        v = max(0, min(v, scale - 1));
        return v;
    };
    return normalize(z0) * scale * scale + normalize(z1) * scale + normalize(z2)
        - 1024 * 1024 * 512;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920.0, 1080.0).into(),
                title: "StudyMS".into(),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
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
    

    for value in res["Layers"].as_array().unwrap() {
        // println!("{:?}", value);
        if value["Tiles"].as_array() != None {
            for tiles in value["Tiles"].as_array().unwrap() {
                // println!("{:?}", tiles);
                println!("{:?}", tiles["Resource"]["ResourceUrl"]);
                let x = tiles["X"].as_f64().unwrap() as f32;
                let y = -tiles["Y"].as_f64().unwrap() as f32 + 330.0;
                // let z = tiles["ID"].as_f64().unwrap() as f32;
                let z = composite_zindex(
                    tiles["Resource"]["Z"].as_i64().unwrap() as i64,
                    tiles["ID"].as_i64().unwrap() as i64,
                    0,
                ) as f32
                    / 100000.0;

                // let ox = tiles["Resource"]["OriginX"].as_f64().unwrap() as f32;
                // let oy = tiles["Resource"]["OriginY"].as_f64().unwrap() as f32;

                let ox = (tiles["Resource"]["OriginX"].as_f64().unwrap() as f32
                    - tiles["Resource"]["Width"].as_f64().unwrap() as f32 / 2.0)
                    / (tiles["Resource"]["Width"].as_f64().unwrap() as f32);

                let oy = -(tiles["Resource"]["OriginY"].as_f64().unwrap() as f32
                    - tiles["Resource"]["Height"].as_f64().unwrap() as f32 / 2.0)
                    / (tiles["Resource"]["Height"].as_f64().unwrap() as f32);

                // println!("{} and {} and {}", x, y, z);
                println!("{} and {}", tiles["ID"].as_i64().unwrap(), z);
                commands
                    .spawn(SpriteBundle {
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
