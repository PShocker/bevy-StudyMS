use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    background::{self, BackGroundEdge},
    player::Player,
};

// 相机最小移动距离，若小于此距离，则移动这个最小距离的长度
const CAMERA_MIN_MOVE_DISTANCE: f32 = 0.1;
// 每帧逼近剩余距离的百分比
const CAMERA_MOVE_INTERPOLATE: f32 = 0.05;

// 相机跟随角色
pub fn camera_follow(
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player: Query<&Transform, With<Player>>,
    mut q_window: Query<&Window, With<PrimaryWindow>>,
    back_ground_edge: Res<BackGroundEdge>,
) {
    if q_player.is_empty() {
        return;
    }
    let player_pos = q_player.single().translation.truncate();
    let camera_pos = q_camera.single().translation.truncate();
    let window = q_window.get_single_mut().ok().unwrap();

    let mut camera_transform = q_camera.single_mut();
    if camera_pos.distance(player_pos) < 0.1 {
        // 视为已达到player位置
        return;
    }
    // if camera_transform.translation.x - window.width() / 2.0 < back_ground_edge.left
    //     && player_pos.x - window.width() / 2.0 < back_ground_edge.left
    // {
    //     // camera_transform.translation.y = player_pos.y;
    //     return;
    // }
    // if camera_transform.translation.x + window.width() / 2.0 > back_ground_edge.right
    //     && player_pos.x + window.width() / 2.0 > back_ground_edge.right
    // {
    //     // camera_transform.translation.y = player_pos.y;
    //     return;
    // }
    if camera_pos.distance(player_pos) < CAMERA_MIN_MOVE_DISTANCE {
        // 直接移动到player位置
        camera_transform.translation.x = player_pos.x;
        camera_transform.translation.y = player_pos.y;
        return;
    }

    // 相机下一帧位置
    let camera_next_pos = camera_pos + (player_pos - camera_pos) * CAMERA_MOVE_INTERPOLATE;
    camera_transform.translation.x = camera_next_pos.x;
    camera_transform.translation.y = camera_next_pos.y;
}
