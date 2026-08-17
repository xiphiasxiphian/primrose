use std::collections::{HashMap, hash_map::Entry};

use crate::{jade::scene::Scene, util::assets::ManagedResource};

pub type ManagedScene = ManagedResource<Scene>;

pub struct SceneManager
{
    scenes: HashMap<&'static str, ManagedScene>,
    current: &'static str,
}

impl SceneManager
{
    pub fn preloaded(
        iter: impl IntoIterator<Item = (&'static str, ManagedScene)>,
        initial_scene: &'static str,
    ) -> Option<Self>
    {
        let scenes = HashMap::from_iter(iter);
        if !scenes.contains_key(initial_scene)
        {
            return None;
        }

        Some(Self {
            scenes: scenes,
            current: initial_scene,
        })
    }

    pub fn add_scene(&mut self, name: &'static str, scene: ManagedScene) -> bool
    {
        match self.scenes.entry(name)
        {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) =>
            {
                e.insert_entry(scene);
                true
            }
        }
    }

    pub fn current_scene(&mut self) -> &Scene
    {
        self.scenes
            .get_mut(&self.current)
            .expect("Current scene set to invalid scene. This should be impossible")
            .get()
    }

    pub fn current_scene_mut(&mut self) -> &mut Scene
    {
        self.scenes
            .get_mut(&self.current)
            .expect("Current scene set to invalid scene. This should be impossible")
            .get()
    }

    pub fn switch(&mut self, target: &'static str) -> Option<&mut Scene>
    {
        let result = self.scenes.get_mut(target)?.get();
        self.current = target;

        Some(result)
    }

    pub fn switch_with<F>(&mut self, target: &'static str, f: F) -> bool
    where
        F: FnOnce(&mut Scene),
    {
        let Some(new_scene) = self.scenes.get_mut(target)
        else
        {
            return false;
        };
        f(new_scene.get());

        true
    }
}
