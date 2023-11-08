use bevy::prelude::*;
use bevy::time::Time;

use crate::common::AnimationTimer;
use crate::player::Player;

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
    mut q_player: Query<(Entity, &mut AnimationTimer, &mut TextureAtlasSprite), With<Player>>,
    time: Res<Time>,
) {
    for (entity, mut timer, mut sprite) in &mut q_player {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            //切换到下一帧
            sprite.index += 1;
            if sprite.index > 3 {
                //回到第一帧
                sprite.index = 0;
            }
        }
    }
}

//背景动画,背景obj的动画效果
pub fn animate_back(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<&mut AnimateObj>,
) {
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
