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

/*
碰撞检测规则:
   (A.collision_groups().memberships & B.collision_groups().filter) != 0
&& (B.collision_groups().memberships & A.collision_groups().filter) != 0

例如向左行走,需要做从上到下的碰撞检测和从左到右的检测,默认B是地砖,A是玩家


( Group::GROUP_1|Group::GROUP_3 & Group::GROUP_1) !=0
&& (Group::ALL & Group::ALL)

(Group::GROUP_2|Group::GROUP_3 & Group::GROUP_1) !=0
*/

pub fn get_foothold_group(p1: Vec2, p2: Vec2) -> Group {
    //先判断平行
    if p1.y == p2.y {
        if p1.x < p2.x {
            //仅检测从上到下的碰撞
            return Group::GROUP_1;
        } else {
            //仅检测从下到上的碰撞
            return Group::GROUP_2;
        }
    }
    //垂直
    if p1.x == p2.x {
        if p1.y < p2.y {
            //仅检测从左到右的碰撞
            return Group::GROUP_3;
        } else {
            //仅检测从右到左的碰撞
            return Group::GROUP_4;
        }
    }
    //斜面
    return Group::GROUP_1;
}
