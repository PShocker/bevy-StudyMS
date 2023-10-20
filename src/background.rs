use bevy::{prelude::*, window::PrimaryWindow};

pub fn background(
    time: Res<Time>,
    mut commands: Commands,
    mut query_backgroud: Query<&mut BackGround>,
    mut query_transform: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let transform = query_transform.get_single_mut().ok().unwrap().0;

    println!("{:?}", transform.translation);
    // for (mut transform,mut o) in query_transform.iter_mut() {
    //     println!("{:?}", transform.translation);
    // }

    for (mut backgroud) in query_backgroud.iter_mut() {
        // println!("{:?}", time.delta_seconds());
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
