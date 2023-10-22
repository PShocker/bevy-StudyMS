use bevy::{prelude::*, window::PrimaryWindow};

pub fn background(
    time: Res<Time>,
    mut commands: Commands,
    mut query_backgroud: Query<&mut BackGround>,
    mut query_transform: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let transform = query_transform.get_single_mut().ok().unwrap().0;

    // println!("{:?}", time.delta_seconds());
    for (mut backgroud) in query_backgroud.iter_mut() {
        // println!("{:?}", time.delta_seconds());
        // println!("{:?}", backgroud.resource);
        let res: serde_json::Value = serde_json::from_str(&backgroud.resource).unwrap();
        let cx;
        if backgroud.cx==0 {
            cx=res["Width"].as_i64().unwrap() as i32;
        }else {
            cx=backgroud.cx;
        }
        let mut positionOffset_x=0;
        // calculate position
        if (backgroud.tilemode.auto_scroll_x==true) {
            positionOffset_x += backgroud.rx * 5 * (time.delta_seconds()*1000.0) as i32;
            positionOffset_x %= cx;
            println!("{:?}", positionOffset_x);
        }
        // let cx=res || resourceRect.width;

        let base_pos_x = backgroud.x+positionOffset_x;
        let base_pos_y = backgroud.y;

        let tile_count_x = 1;
        let tile_count_y = 1;
        if backgroud.tilemode.tile_x && cx > 0 {
            let tile_start_right = (base_pos_x + resourceRect.right - screenRect.left) % cx;
            if (tile_start_right <= 0)
                tile_start_right += cx;
            tile_start_right += screenRect.left;

            let tileStartLeft = tile_start_right - resourceRect.width;
            if (tileStartLeft >= screenRect.right) {
                tile_count_x = 0;
            } else {
                tile_count_x = Math.ceil((screenRect.right - tileStartLeft) / cx);
                basePos.x = tileStartLeft - resourceRect.x;
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
