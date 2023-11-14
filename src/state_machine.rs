use bevy::{
    prelude::{Bundle, Component, EventWriter, Input, KeyCode, Query, Res, ResMut, Vec2, With},
    sprite::TextureAtlasSprite,
    time::Timer,
};
use bevy_rapier2d::prelude::Velocity;

use crate::{
    animate::{AnimationIndices, AnimationTimer},
    customfilter::CustomFilterTag,
    player::{Direction, Player, PlayerGrounded, PlayerState, StateChangeEvent},
};

pub fn player_state_machine(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<(&mut Velocity, &mut CustomFilterTag), With<Player>>,
    mut player_state: ResMut<PlayerState>,
    player_grounded: Res<PlayerGrounded>,
) {
    if q_player.is_empty() {
        return;
    }

    for (mut velocity, mut group) in &mut q_player {
        // Jumping状态
        if !player_grounded.flag {
            *player_state = PlayerState::Jumping;
            return;
        }

        // Standing状态
        if player_grounded.flag
            && velocity.linvel.x.abs() < 0.1
            && *player_state != PlayerState::Prone
        {
            *player_state = PlayerState::Standing;
            return;
        }
        // Running状态
        if player_grounded.flag && velocity.linvel.x.abs() > 1.0 {
            *player_state = PlayerState::Walking;
            return;
        }
    }
}

pub fn player_gravity_machine(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<(&mut Velocity, &mut CustomFilterTag), With<Player>>,
    mut player_state: ResMut<PlayerState>,
    player_grounded: Res<PlayerGrounded>,
) {
    if q_player.is_empty() {
        return;
    }

    for (mut velocity, mut group) in &mut q_player {
        //下落最大速度
        // println!("{:?},{:?}", velocity.linvel.x, velocity.linvel.y);
        // if !player_grounded.flag {
        //     velocity.linvel.y -= 10.0;
        // } 
        // else if player_grounded.flag && velocity.linvel.y < 0.0 {
        //     velocity.linvel.y = 0.0;
        // }

        if velocity.linvel.y <= -220.0 {
            velocity.linvel.y = -220.0;
        }
        // // //判断下跳
        // if velocity.linvel.y <= -40.0 {
        //     //下落
        //     *group = CustomFilterTag::GroupA;
        // }
        // if velocity.linvel.y >= 10.0 {
        //     *group = CustomFilterTag::GroupB;
        // }

        // if velocity.linvel.y >= 10.0 {
        //     *group = CustomFilterTag::GroupB;
        // }
        // if velocity.linvel.y >= 10.0 {
        //     *group = CustomFilterTag::GroupB;
        // }
    }
}

