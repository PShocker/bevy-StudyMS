use bevy::{prelude::*, render::render_phase::PhaseItem, window::PrimaryWindow};

#[derive(Component, Debug)]
pub struct FootHold {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub prev: i32,
    pub next: i32,
    pub piece: i32,
    pub id: i32,
}
