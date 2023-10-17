//! Displays a single [`Sprite`], created from an image.

use bevy::{prelude::*, window::WindowMode};
use std::{
    cmp::{max, min},
    fs, time,
};

fn composite_zindex(z: i128, z0: i128, z1: i128, z2: i128) -> i128 {
    let scale = 1 << 10; // 1024
    let normalize = |mut v: i128| -> i128 {
        // v = v.abs();
        v = v + scale / 2;
        v = max(0, min(v, scale - 1));
        return v;
    };
    return normalize(z) * scale * scale * scale
        + normalize(z0) * scale * scale
        + normalize(z1) * scale
        + normalize(z2)
        - 1024 * 1024 * 1024 * 512;
}

#[derive(Component)]
pub struct AnimationSprite {
    pub sprite: Vec<SpriteBundle>,
    pub index: i32,
    pub delays: Vec<f32>,
    pub delay: f32,
    pub start: bool,
    pub lastsprite: Option<Entity>,
}
#[derive(Component)]
pub struct Name(String);

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
        .add_systems(Update, animation)
        .run();
}

pub fn animation(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<&mut AnimationSprite, With<Name>>,
) {
    // println!("{:?}", time.raw_elapsed_seconds());

    for mut s in &mut query {
        if s.index == -1 {
            s.index += 1;
            s.lastsprite = Some(commands.spawn(s.sprite[0].to_owned()).id());
            s.delay = s.delays[s.index as usize] / 1000.0 + time.raw_elapsed_seconds();
            s.start = true;
        } else {
            if s.lastsprite != None {
                if s.start == true {
                    if time.raw_elapsed_seconds() >= s.delay {
                        commands.entity(s.lastsprite.unwrap()).despawn();
                        s.lastsprite =
                            Some(commands.spawn(s.sprite[s.index as usize].to_owned()).id());
                        if s.index as usize == s.sprite.len() - 1 {
                            s.index = 0;
                        } else {
                            s.index += 1;
                        }
                        s.delay = s.delays[s.index as usize] / 1000.0 + time.raw_elapsed_seconds();
                    }
                }
            }
        }

        // println!("{:?}", s.sprite);
    }
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
                let y = -objs["Y"].as_f64().unwrap() as f32 + 330.0;
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
                        let ox = (frames["OriginX"].as_f64().unwrap() as f32
                            - frames["Width"].as_f64().unwrap() as f32 / 2.0)
                            / (frames["Width"].as_f64().unwrap() as f32);

                        let oy = -(frames["OriginY"].as_f64().unwrap() as f32
                            - frames["Height"].as_f64().unwrap() as f32 / 2.0)
                            / (frames["Height"].as_f64().unwrap() as f32);
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

                    commands.spawn((animationsprite, Name(i.to_string())));
                    // commands.
                }
            }
        }

        if value["Tiles"].as_array() != None {
            for tiles in value["Tiles"].as_array().unwrap() {
                // println!("{:?}", tiles);
                // println!("{:?}", tiles["Resource"]["ResourceUrl"]);
                let x = tiles["X"].as_f64().unwrap() as f32;
                let y = -tiles["Y"].as_f64().unwrap() as f32 + 330.0;
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

                let ox = (tiles["Resource"]["OriginX"].as_f64().unwrap() as f32
                    - tiles["Resource"]["Width"].as_f64().unwrap() as f32 / 2.0)
                    / (tiles["Resource"]["Width"].as_f64().unwrap() as f32);

                let oy = -(tiles["Resource"]["OriginY"].as_f64().unwrap() as f32
                    - tiles["Resource"]["Height"].as_f64().unwrap() as f32 / 2.0)
                    / (tiles["Resource"]["Height"].as_f64().unwrap() as f32);

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
            let resource = backs["Resource"].as_object().unwrap();
            println!("{:?}", resource);
            match backs["Ani"].as_i64().unwrap() {
                0 => {
                    //sprite
                    let x = backs["X"].as_f64().unwrap() as f32;
                    let y = -backs["Y"].as_f64().unwrap() as f32 + 330.0;
                    let z = backs["ID"].as_f64().unwrap() as f32 / 100.0;
                    // let ox = tiles["Resource"]["OriginX"].as_f64().unwrap() as f32;
                    // let oy = tiles["Resource"]["OriginY"].as_f64().unwrap() as f32;

                    let ox = (backs["Resource"]["OriginX"].as_f64().unwrap() as f32
                        - backs["Resource"]["Width"].as_f64().unwrap() as f32 / 2.0)
                        / (backs["Resource"]["Width"].as_f64().unwrap() as f32);

                    let oy = -(backs["Resource"]["OriginY"].as_f64().unwrap() as f32
                        - backs["Resource"]["Height"].as_f64().unwrap() as f32 / 2.0)
                        / (backs["Resource"]["Height"].as_f64().unwrap() as f32);

                    // println!("{} and {} and {}", x, y, z);
                    // println!("{} and {}", tiles["ID"].as_i64().unwrap(), z);
                    commands.spawn(SpriteBundle {
                        texture: asset_server.load(
                            backs["Resource"]["ResourceUrl"]
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
                1 => {}
                _ => println!("Ani Other"),
            }
            // print!("{:?}", backs);
        }
    }
}
