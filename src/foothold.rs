use bevy::{prelude::*, render::render_phase::PhaseItem, window::PrimaryWindow};

use crate::customfilter::CustomFilterTag;

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

pub fn get_foothold_group(p1: Vec2, p2: Vec2) -> CustomFilterTag {
    //先判断平行
    if p1.y == p2.y {
        if p1.x < p2.x {
            return CustomFilterTag::GroupA;
        } else {
            return CustomFilterTag::GroupB;
        }
    }
    //垂直
    if p1.x == p2.x {
        if p1.y < p2.y {
            return CustomFilterTag::GroupC;
        } else {
            return CustomFilterTag::GroupD;
        }
    }
    return CustomFilterTag::GroupA;
}
