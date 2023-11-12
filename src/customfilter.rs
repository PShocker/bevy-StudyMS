use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Component)]
pub enum CustomFilterTag {
    GroupA,
    GroupB,
}

// A custom filter that allows contacts only between rigid-bodies with the
// same user_data value.
// Note that using collision groups would be a more efficient way of doing
// this, but we use custom filters instead for demonstration purpose.
#[derive(SystemParam)]
pub struct SameUserDataFilter<'w, 's> {
    tags: Query<'w, 's, &'static CustomFilterTag>,
}

static mut ENITY: Option<Entity> = None;

impl BevyPhysicsHooks for SameUserDataFilter<'_, '_> {
    fn filter_contact_pair(&self, context: PairFilterContextView) -> Option<SolverFlags> {
        if self.tags.get(context.collider1()).ok().copied()
            == self.tags.get(context.collider2()).ok().copied()
        {
            return Some(SolverFlags::COMPUTE_IMPULSES);
        } else {
            if unsafe { ENITY } == None {
                unsafe { ENITY = Some(context.collider1()) };
                return None;
            } else if unsafe { ENITY.unwrap() } != context.collider1() {
                return Some(SolverFlags::COMPUTE_IMPULSES);
            } else {
                return None;
            }
        }
    }
}
