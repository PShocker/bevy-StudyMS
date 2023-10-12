use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct MapInfo {
    pub ID: u8,
    pub Layers: Vec<Layers>,
    pub Backs: Vec<Backs>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layers {
    Tiles: Option<Vec<Tiles>>,
    Objs: Option<Vec<Objs>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tiles {
    ID: i32,
    X: i32,
    Y: i32,
    Resource: Resource,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resource {
    Width: i32,
    Height: i32,
    OriginX: i32,
    OriginY: i32,
    Z: i32,
    ResourceUrl: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Backs {
    ID: i32,
    X: i32,
    Y: i32,
    Cx: i32,
    Cy: i32,
    Rx: i32,
    Ry: i32,
    Alpha: i32,
    FlipX: bool,
    Front: bool,
    Ani: i32,
    Type: i32,
    Resource: Resource,
}

#[derive(Debug, Serialize, Deserialize)]
struct Objs {
    ID: i32,
    X: i32,
    Y: i32,
    Z: i32,
    FlipX: bool,
    Resource: Option<Resource2>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resource2 {
    Frames: Option<Vec<Frames>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Frames {
    Delay: i32,
    A0: i32,
    A1: i32,
    Width: i32,
    Height: i32,
    OriginX: i32,
    OriginY: i32,
    Z: i32,
    ResourceUrl: String,
}
