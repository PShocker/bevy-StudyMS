use bevy::{prelude::{Component, Bundle}, time::Timer};

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationIndices {
    // 表示sprite_indices数组下标
    pub index: usize,
    pub sprite_indices: Vec<usize>,
}

#[derive(Clone, Debug, Default, Bundle)]
pub struct AnimationBundle {
    pub timer: AnimationTimer,
    pub indices: AnimationIndices,
}
