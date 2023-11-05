use bevy::{prelude::*, render::render_phase::PhaseItem, window::PrimaryWindow};

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

pub fn foothold(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<&mut FootHold>,
) {
    // println!("{:?}", time.raw_elapsed_seconds());

    for mut s in &mut query {
        // println!("{:?}", s);
        gizmos.line_2d(
            Vec2::new(s.x1 as f32, -s.y1 as f32),
            Vec2::new(s.x2 as f32, -s.y2 as f32),
            Color::RED,
        );
    }
}
