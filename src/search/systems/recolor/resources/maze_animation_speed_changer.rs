use bevy::prelude::Component;

#[derive(Component)]
pub enum MazeAnimationSpeedChanger {
    Slower,
    Faster,
}
