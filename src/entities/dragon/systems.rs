use std::f32::consts::PI;

use bevy::{ecs::query::WorldQuery, prelude::*};

use crate::{
    direction::Direction,
    grid::GridPosition,
    level::{LevelComponent, WinTimer},
    movement::Movement,
};

use super::{assets::DragonAssets, components::DragonHead};

#[derive(WorldQuery)]
pub struct SpawnBodyDragonQuery<'w> {
    position: &'w GridPosition,
    direction: &'w Direction,
    movement: &'w Movement,
}

pub fn spawn_body(
    mut commands: Commands,
    assets: Res<DragonAssets>,
    dragons: Query<SpawnBodyDragonQuery, (With<DragonHead>, Changed<Movement>)>,
) {
    for dragon in dragons.iter() {
        if dragon.movement.0.is_none() {
            continue;
        }

        commands
            .spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..Default::default()
                },
                texture_atlas: assets.atlas.clone(),
                ..Default::default()
            })
            .insert(LevelComponent)
            .insert(*dragon.position)
            .insert(*dragon.direction);
    }
}

pub fn rotate_dragons(mut q: Query<(&Direction, &mut Transform), Changed<Direction>>) {
    for (direction, mut transform) in q.iter_mut() {
        transform.rotation = Quat::from_rotation_z(
            (PI / 180.0)
                * match direction {
                    Direction::Up => 270.0,
                    Direction::Down => 90.0,
                    Direction::Left => 0.0,
                    Direction::Right => 180.0,
                },
        );
    }
}

pub fn check_win(
    mut commands: Commands,
    dragons: Query<(&GridPosition, &Direction), With<DragonHead>>,
) {
    let dragons_opposite = dragons
        .iter_combinations::<2>()
        .any(|[a, b]| a.0.apply_direction(*a.1) == *b.0 && a.1.opposite() == *b.1);

    if dragons_opposite {
        commands.insert_resource(WinTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    }
}
