use bevy::prelude::{App, Mut, World};

struct Loadables<Scene>(pub Vec<fn(&mut World, &Scene)>);

pub trait Loadable<Scene> {
    fn from_scene(world: &mut World, scene: &Scene);
}

pub trait AppLoadableExt<Scene> {
    fn register_loadable<T: Loadable<Scene>>(&mut self) -> &mut Self;
}

impl<Scene: 'static> AppLoadableExt<Scene> for App {
    fn register_loadable<T: Loadable<Scene>>(&mut self) -> &mut Self {
        let mut loadables = self
            .world
            .get_resource_or_insert_with(|| Loadables(Vec::new()));
        loadables.0.push(T::from_scene);

        self
    }
}

pub fn load_loadables<Scene: 'static>(world: &mut World, scene: &Scene) {
    world.resource_scope(|world, loadables: Mut<Loadables<Scene>>| {
        for loadable in &loadables.0 {
            loadable(world, scene);
        }
    });
}
