use bevy::{prelude::*, render::render_phase::PhaseItem, window::PrimaryWindow};
use bevy_rapier2d::geometry::Group;

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

pub fn get_foothold_group(p1: Vec2, p2: Vec2) -> Group {
    //先判断平行
    if p1.y == p2.y {
        if p1.x < p2.x {
            return Group::GROUP_1;
        } else {
            return Group::GROUP_2;
        }
    }
    //垂直
    if p1.x == p2.x {
        if p1.y < p2.y {
            return Group::GROUP_3;
        } else {
            return Group::GROUP_4;
        }
    }
    return Group::GROUP_1;
}
