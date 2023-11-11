use bevy::prelude::*;
use bevy::time::Time;


use crate::player::{Player, PlayerState, PlayerStateAnimate, StateChangeEvent};

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationIndices {
    pub index: usize,
    pub sprite_indices: Vec<usize>,
}

#[derive(Clone, Debug, Default, Bundle)]
pub struct AnimationBundle {
    pub timer: AnimationTimer,
    pub indices: AnimationIndices,
}


//
#[derive(Component)]
pub struct AnimateObj {
    pub sprite: Vec<SpriteBundle>,
    pub index: i32,
    pub delays: Vec<f32>,
    pub delay: f32,
    pub start: bool,
    pub lastsprite: Option<Entity>,
}

//播放人物行走动画
pub fn animate_player(
    mut commands: Commands,
    mut q_player: Query<
        (
            Entity,
            &mut AnimationTimer,
            &mut AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut state_change_ev: EventReader<StateChangeEvent>,
) {
    for (entity, mut timer, mut indices, mut sprite) in &mut q_player {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = if indices.index == indices.sprite_indices.len() - 1 {
                indices.index = 0;
                indices.sprite_indices[indices.index]
            } else {
                indices.index += 1;
                indices.sprite_indices[indices.index]
            };
        } else if state_change_ev.iter().next().is_some() {
            sprite.index = if indices.index == indices.sprite_indices.len() - 1 {
                indices.index = 0;
                indices.sprite_indices[indices.index]
            } else {
                indices.index += 1;
                indices.sprite_indices[indices.index]
            };
        }
    }
}

//背景动画,背景obj的动画效果
pub fn animate_back(time: Res<Time>, mut commands: Commands, mut query: Query<&mut AnimateObj>) {
    // println!("{:?}", time.raw_elapsed_seconds());

    for mut s in &mut query {
        // println!("{:?}", s);
        if s.index == -1 {
            s.index += 1;
            s.lastsprite = Some(commands.spawn(s.sprite[0].to_owned()).id());
            s.delay = s.delays[s.index as usize] / 1000.0 + time.raw_elapsed_seconds();
            s.start = true;
        } else {
            if s.lastsprite != None {
                if s.start == true {
                    if time.raw_elapsed_seconds() >= s.delay {
                        commands.entity(s.lastsprite.unwrap()).despawn();
                        s.lastsprite =
                            Some(commands.spawn(s.sprite[s.index as usize].to_owned()).id());
                        if s.index as usize == s.sprite.len() - 1 {
                            s.index = 0;
                        } else {
                            s.index += 1;
                        }
                        s.delay = s.delays[s.index as usize] / 1000.0 + time.raw_elapsed_seconds();
                    }
                }
            }
        }

        // println!("{:?}", s.sprite);
    }
}
