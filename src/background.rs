use bevy::{prelude::*, window::PrimaryWindow};

use crate::utils::{cal_ax, cal_ay};

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct BackEnity;

#[derive(Debug, Resource)]
pub struct BackGroundEdge {
    pub left: f32,
    pub right: f32,
}

pub struct BackGroundPlugin;

impl Plugin for BackGroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, background);
    }
}

//绘制背景,且背景随人物移动
fn background(
    time: Res<Time>,
    mut commands: Commands,
    mut q_backgroud: Query<&mut BackGround>,
    mut q_transform: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_backenity: Query<Entity, With<BackEnity>>,
    asset_server: Res<AssetServer>,
    mut last: Local<Vec3>,
) {
    let transform = q_transform.get_single_mut().ok().unwrap().0;
    let window = q_window.get_single_mut().ok().unwrap();

    if last.eq(&transform.translation) {
        return;
    } else {
        *last = transform.translation;
    }
    
    for backenity in q_backenity.iter_mut() {
        commands.entity(backenity).despawn();
    }

    for backgroud in q_backgroud.iter_mut() {
        let res: serde_json::Value = serde_json::from_str(&backgroud.resource).unwrap();
        let cx;
        let cy;
        if backgroud.cx == 0 {
            cx = res["Width"].as_i64().unwrap() as i32;
        } else {
            cx = backgroud.cx;
        }

        if backgroud.cy == 0 {
            cy = res["Height"].as_i64().unwrap() as i32;
        } else {
            cy = backgroud.cy;
        }
        let mut position_offset_x = 0;
        let mut position_offset_y = 0;

        if backgroud.tilemode.auto_scroll_x == true {
            position_offset_x += (backgroud.rx as f32 * 5.0 * time.delta_seconds()) as i32;
            position_offset_x %= cx;
        } else {
            position_offset_x =
                (transform.translation.x as f32 * (backgroud.rx + 100) as f32 / 100.0) as i32;
        }

        if backgroud.tilemode.auto_scroll_y == true {
            position_offset_y += (backgroud.ry as f32 * 5.0 * time.delta_seconds()) as i32;
            position_offset_y %= cy;
        } else {
            position_offset_y =
                (transform.translation.y as f32 * (backgroud.ry + 100) as f32 / 100.0) as i32;
        }

        let mut base_pos_x = backgroud.x + position_offset_x;
        let mut base_pos_y = backgroud.y + position_offset_y;

        let mut x = base_pos_x as f32;
        let mut y = base_pos_y as f32;
        let mut z: f32;

        let mut tile_count_x = 1;
        let mut tile_count_y = 1;
        let screen_left = transform.translation.x as i32 - window.width() as i32 / 2;
        // let screen_left = 0;
        let screen_right = screen_left + window.width() as i32;

        let screen_top = transform.translation.y as i32 + window.height() as i32 / 2;
        let screen_bottom = screen_top - window.height() as i32;

        if backgroud.tilemode.tile_x && cx > 0 {
            if x <= screen_left as f32 {
                while x <= screen_left as f32 {
                    x += cx as f32;
                }
                x -= cx as f32;
            } else {
                while x > screen_left as f32 {
                    x -= cx as f32;
                }
            }
            tile_count_x += (screen_right - x as i32) / cx + 1;
        }

        if backgroud.tilemode.tile_y && cy > 0 {
            if y <= screen_bottom as f32 {
                while y <= screen_bottom as f32 {
                    y += cy as f32;
                }
                y -= cy as f32;
            } else {
                while y > screen_bottom as f32 {
                    y -= cy as f32;
                }
            }
            tile_count_y += (screen_top - y as i32) / cy + 1;
        }

        // println!("tile_count_y:{:?}", tile_count_y);

        if backgroud.front == true {
            z = -10.0 + backgroud.id as f32 / 10.0;
        } else {
            z = -20.0 + backgroud.id as f32 / 10.0;
        }

        let ox = cal_ax(
            res["OriginX"].as_f64().unwrap() as f32,
            res["Width"].as_f64().unwrap() as f32,
        );

        let oy = -cal_ay(
            res["OriginY"].as_f64().unwrap() as f32,
            res["Height"].as_f64().unwrap() as f32,
        );

        for j in 0..tile_count_y {
            for i in 0..tile_count_x {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server
                            .load(res["ResourceUrl"].to_string().replace("\"", "")),
                        transform: Transform::from_xyz(x + (i * cx) as f32, y + (j * cy) as f32, z),
                        sprite: Sprite {
                            anchor: bevy::sprite::Anchor::Custom(Vec2::new(ox, oy)),
                            ..default()
                        },
                        ..default()
                    },
                    BackEnity,
                ));
            }
        }
    }
}

#[derive(Component)]
pub struct BackGround {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub rx: i32,
    pub ry: i32,
    pub alpha: i32,
    pub flip_x: bool,
    pub front: bool,
    pub ani: i32,
    pub types: i32,
    pub resource: String,
    pub tilemode: Tilemode,
}

pub struct Tilemode {
    pub tile_x: bool,
    pub tile_y: bool,
    pub auto_scroll_x: bool,
    pub auto_scroll_y: bool,
}

impl BackGround {
    pub fn new(
        id: i32,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        rx: i32,
        ry: i32,
        alpha: i32,
        flip_x: bool,
        front: bool,
        ani: i32,
        types: i32,
        resource: String,
    ) -> BackGround {
        let tilemode;
        match types {
            0 => {
                tilemode = Tilemode {
                    tile_x: false,
                    tile_y: false,
                    auto_scroll_x: false,
                    auto_scroll_y: false,
                };
            }
            1 => {
                tilemode = Tilemode {
                    tile_x: true,
                    tile_y: false,
                    auto_scroll_x: false,
                    auto_scroll_y: false,
                };
            }
            2 => {
                tilemode = Tilemode {
                    tile_x: false,
                    tile_y: true,
                    auto_scroll_x: false,
                    auto_scroll_y: false,
                };
            }
            3 => {
                tilemode = Tilemode {
                    tile_x: true,
                    tile_y: true,
                    auto_scroll_x: false,
                    auto_scroll_y: false,
                };
            }
            4 => {
                tilemode = Tilemode {
                    tile_x: true,
                    tile_y: false,
                    auto_scroll_x: true,
                    auto_scroll_y: false,
                };
            }
            5 => {
                tilemode = Tilemode {
                    tile_x: false,
                    tile_y: true,
                    auto_scroll_x: false,
                    auto_scroll_y: true,
                };
            }
            6 => {
                tilemode = Tilemode {
                    tile_x: true,
                    tile_y: true,
                    auto_scroll_x: true,
                    auto_scroll_y: false,
                };
            }
            7 => {
                tilemode = Tilemode {
                    tile_x: true,
                    tile_y: true,
                    auto_scroll_x: false,
                    auto_scroll_y: true,
                };
            }
            _ => {
                tilemode = Tilemode {
                    tile_x: false,
                    tile_y: false,
                    auto_scroll_x: false,
                    auto_scroll_y: false,
                };
            }
        }
        BackGround {
            id,
            x,
            y,
            cx,
            cy,
            rx,
            ry,
            alpha,
            flip_x,
            front,
            ani,
            types,
            resource,
            tilemode,
        }
    }
}
