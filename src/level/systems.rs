use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{util, State};

use super::{
    assets::LevelAssets,
    components::LevelComponent,
    config::LevelConfig,
    resources::{CurrentLevel, WinTimer},
};

pub fn load_level(world: &mut World) {
    world.resource_scope(|world, config: Mut<LevelAssets>| {
        let index = world.resource_scope(|_, mut current: Mut<CurrentLevel>| {
            let index = current.0 % config.levels.len();
            current.0 += 1;
            index
        });

        world.resource_scope(|world, assets: Mut<Assets<LevelConfig>>| {
            let handle = &config.levels[index];
            let level = assets.get(handle).unwrap();

            util::load_loadables(world, level);
        });
    });

    world.insert_resource(NextState(State::InLevel));
}

pub fn check_win_timer(mut commands: Commands, time: Res<Time>, mut timer: ResMut<WinTimer>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        commands.insert_resource(NextState(State::LevelLoading));
    }
}

pub fn unload_level(mut commands: Commands, mut level_query: Query<Entity, With<LevelComponent>>) {
    commands.remove_resource::<WinTimer>();

    for item in level_query.iter_mut() {
        commands.entity(item).despawn();
    }
}
