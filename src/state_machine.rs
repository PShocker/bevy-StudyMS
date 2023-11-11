use bevy::{
    prelude::{Bundle, Component, EventWriter, Input, KeyCode, Query, Res, ResMut, With},
    sprite::TextureAtlasSprite,
    time::Timer,
};
use bevy_rapier2d::prelude::Velocity;

use crate::{
    animate::{AnimationIndices, AnimationTimer},
    player::{Facing, Player, PlayerGrounded, PlayerState, PlayerStateAnimate, StateChangeEvent},
};

pub fn player_state_machine(
    keyboard_input: Res<Input<KeyCode>>,
    q_player: Query<&Velocity, With<Player>>,
    mut player_state: ResMut<PlayerState>,
    player_grounded: Res<PlayerGrounded>,
) {
    if q_player.is_empty() {
        return;
    }
    let velocity = q_player.single();

    // Jumping状态
    if !player_grounded.flag {
        *player_state = PlayerState::Jumping;
        return;
    }

    // Standing状态
    if player_grounded.flag && velocity.linvel.x.abs() < 0.1 {
        *player_state = PlayerState::Standing;
        return;
    }
    // Running状态
    if player_grounded.flag && velocity.linvel.x.abs() > 1.0 {
        *player_state = PlayerState::Walking;
        return;
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
                    *state=PlayerState::Standing;
                    *indices = player_ani.stand.indices.clone();
                    *timer=player_ani.stand.timer.clone();
                    state_change_ev.send_default();
                }
            }
            PlayerState::Walking => {
                if  *state != PlayerState::Walking{
                    *state=PlayerState::Walking;
                    *indices = player_ani.walk.indices.clone();
                    *timer=player_ani.walk.timer.clone();
                    state_change_ev.send_default();
                }
            }
            PlayerState::Jumping => {
                if  *state != PlayerState::Jumping{
                    *state=PlayerState::Jumping;
                    *indices = player_ani.jump.indices.clone();
                    *timer=player_ani.jump.timer.clone();
                    state_change_ev.send_default();
                }
            }
        }
    }
}
