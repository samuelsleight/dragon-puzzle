use bevy::time::Timer;

#[derive(Clone, Debug)]
pub struct WinTimer(pub Timer);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentLevel(pub usize);
