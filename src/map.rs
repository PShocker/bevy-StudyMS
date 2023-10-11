use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Map {
    ID: u8,
    Layers: Vec<>,
    Backs: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Layers {
    Tiles: Vec<Tiles>,
    Objs: Vec<Objs>,
}

#[derive(Serialize, Deserialize)]
struct Tiles {
    ID: u32,
    X: u32,
    Y: u32,
    Resource:Resource
}

#[derive(Serialize, Deserialize)]
struct Objs {
    ID: u32,
    X: u32,
    Y: u32,
    Resource:Resource
}

#[derive(Serialize, Deserialize)]
struct Resource {
    Width: u32,
    Height: u32,
    OriginX: u32,
    OriginY: u32,
    Z: u32,
    ResourceUrl: String,
}