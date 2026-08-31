use std::{
    any::{Any, TypeId},
    collections::{HashMap, hash_map::Entry},
};

use crate::{
    jade::{ecs::resource::Resource, scene::Scene},
    util::assets::ManagedResource,
};

pub type ManagedScene = ManagedResource<Scene>;

pub struct SceneManager
{
    scenes: HashMap<&'static str, ManagedScene>,
    current: &'static str,
    global_resources: HashMap<TypeId, Box<dyn Resource>>,
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
            global_resources: HashMap::new(),
        })
    }

    pub fn with_global<R: Resource>(mut self, resource: R) -> Self
    {
        self.global_resources.insert(TypeId::of::<R>(), Box::new(resource));

        self
    }

    pub fn global<R: Resource>(&self) -> Option<&R>
    {
        self.global_resources
            .get(&TypeId::of::<R>())
            .and_then(|r| (r.as_ref() as &dyn Any).downcast_ref::<R>())
    }

    pub fn global_mut<R: Resource>(&mut self) -> Option<&mut R>
    {
        self.global_resources
            .get_mut(&TypeId::of::<R>())
            .and_then(|r| (r.as_mut() as &mut dyn Any).downcast_mut::<R>())
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
