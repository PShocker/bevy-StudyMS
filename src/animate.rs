use bevy::prelude::*;
use bevy::time::Time;

use crate::player::{Player, StateChangeEvent};

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationIndices {
    pub index: usize,
    pub sprite_indices: Vec<usize>,
}

#[derive(Clone, Debug, Default, Component)]
pub struct Animation {
    pub timer: AnimationTimer,
    pub indices: AnimationIndices,
    pub name: String,
}

//
#[derive(Component)]
pub struct Animations {
    pub sprite: Vec<SpriteBundle>,
    pub index: i32,
    pub delays: Vec<f32>,
    pub delay: f32,
    pub start: bool,
    pub lastsprite: Option<Entity>,
}

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,animate_player);
        app.add_systems(Update,animate_back);
    }
}


//播放人物行走动画
fn animate_player(
    mut commands: Commands,
    mut q_player: Query<
        (
            Entity,
            &mut Animation,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut state_change_ev: EventReader<StateChangeEvent>,
) {
    for (entity, mut animation, mut sprite) in &mut q_player {
        // println!("{:?}",animation.name);
        if animation.timer.0.tick(time.delta()).just_finished() ||!state_change_ev.is_empty(){
            let current_idx = animation
                .indices.sprite_indices.iter()
                .position(|s| *s == sprite.index)
                .unwrap_or(0); // default to 0 if the current sprite is not in the set

            let next_idx = (current_idx + animation.timer.0.times_finished_this_tick() as usize)
                % animation.indices.sprite_indices.len();

            sprite.index = animation.indices.sprite_indices[next_idx];
            // state_change_ev.clear();
        }
    }
}

//背景动画,背景obj的动画效果
fn animate_back(time: Res<Time>, mut commands: Commands, mut query: Query<&mut Animations>) {
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
