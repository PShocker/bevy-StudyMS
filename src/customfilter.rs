use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Component)]
pub enum CustomFilterTag {
    GroupA=1,
    GroupB=2,
    GroupC=4,
    GroupD=8,
}

// // A custom filter that allows contacts only between rigid-bodies with the
// // same user_data value.
// // Note that using collision groups would be a more efficient way of doing
// // this, but we use custom filters instead for demonstration purpose.
// #[derive(SystemParam)]
// pub struct SameUserDataFilter<'w, 's> {
//     tags: Query<'w, 's, &'static CustomFilterTag>,
// }

// impl BevyPhysicsHooks for SameUserDataFilter<'_, '_> {
//     fn filter_contact_pair(&self, context: PairFilterContextView) -> Option<SolverFlags> {
//         // println!("{:?}", self.tags);
//         if self.tags.get(context.collider1()).ok().copied()
//             == self.tags.get(context.collider2()).ok().copied()
//         {
//             // println!("{:?}", self.tags);
//             return Some(SolverFlags::COMPUTE_IMPULSES);
//         } else {
//             // println!("{:?}", self.tags);
//             return None;
//         }
//     }
// }
