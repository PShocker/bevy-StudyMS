use bevy::{
    prelude::{Bundle, Component, EventWriter, Input, KeyCode, Query, Res, ResMut, Vec2, With},
    sprite::TextureAtlasSprite,
    time::Timer,
};
use bevy_rapier2d::prelude::Velocity;

use crate::{
    animate::{AnimationIndices, AnimationTimer},
    customfilter::CustomFilterTag,
    player::{Facing, Player, PlayerGrounded, PlayerState, PlayerStateAnimate, StateChangeEvent},
};

pub fn player_state_machine(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<(&Velocity, &mut CustomFilterTag), With<Player>>,
    mut player_state: ResMut<PlayerState>,
    player_grounded: Res<PlayerGrounded>,
) {
    if q_player.is_empty() {
        return;
    }

    for (mut velocity, mut group) in &mut q_player {
        //
        println!("{:?}",velocity.linvel.y);
        if *group == CustomFilterTag::GroupB && velocity.linvel.y < -150.0 {
            *group = CustomFilterTag::GroupA;
        } else if velocity.linvel.y >= 10.0 {
            *group = CustomFilterTag::GroupB;
        }
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

pub fn player_sprite_machine(
    mut player_state: ResMut<PlayerState>,
    mut q_player: Query<
        (
            &mut TextureAtlasSprite,
            &mut AnimationIndices,
            &mut AnimationTimer,
            &mut PlayerState,
        ),
        With<Player>,
    >,
    player_ani: Res<PlayerStateAnimate>,
    mut state_change_ev: EventWriter<StateChangeEvent>,
    mut player_grounded: ResMut<PlayerGrounded>,
) {
    if q_player.is_empty() {
        return;
    }
    for (mut sprite, mut indices, mut timer, mut state) in &mut q_player {
        match *player_state {
            PlayerState::Standing => {
                if *state != PlayerState::Standing {
                    *state = PlayerState::Standing;
                    *indices = player_ani.stand.indices.clone();
                    *timer = player_ani.stand.timer.clone();
                    state_change_ev.send_default();
                }
            }
            PlayerState::Walking => {
                if *state != PlayerState::Walking {
                    *state = PlayerState::Walking;
                    *indices = player_ani.walk.indices.clone();
                    *timer = player_ani.walk.timer.clone();
                    state_change_ev.send_default();
                }
            }
            PlayerState::Jumping => {
                if *state != PlayerState::Jumping {
                    *state = PlayerState::Jumping;
                    *indices = player_ani.jump.indices.clone();
                    *timer = player_ani.jump.timer.clone();
                    state_change_ev.send_default();
                }
            }
            PlayerState::Prone => {
                if *state != PlayerState::Prone {
                    *state = PlayerState::Prone;
                    *indices = player_ani.prone.indices.clone();
                    *timer = player_ani.prone.timer.clone();
                    state_change_ev.send_default();
                }
            }
        }
    }
}
