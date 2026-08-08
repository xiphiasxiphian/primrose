use std::{
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
    mem::{self, zeroed},
};

use crate::jade::scene::Scene;

pub struct ManagedScene(ManagedSceneInner);
enum ManagedSceneInner
{
    Resolved(Scene),
    Lazy(Box<dyn FnOnce() -> Scene>),

    #[doc(hidden)]
    Evaluating,
}

impl ManagedSceneInner
{
    pub fn resolve(&mut self) -> &mut Scene
    {
        if let Self::Lazy(_) = self
        {
            let old = std::mem::replace(self, Self::Evaluating);
            let scene = match old
            {
                Self::Lazy(f) => f(),
                _ => unreachable!(),
            };

            *self = Self::Resolved(scene);
        }

        match self
        {
            Self::Resolved(s) => s,
            _ => unreachable!(),
        }
    }
}

impl ManagedScene
{
    pub(super) fn resolve(&mut self) -> &mut Scene { self.0.resolve() }

    pub fn eager(scene: Scene) -> Self { Self(ManagedSceneInner::Resolved(scene)) }

    pub fn lazy(f: Box<dyn FnOnce() -> Scene>) -> Self { Self(ManagedSceneInner::Lazy(f)) }
}

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
            .resolve()
    }

    pub fn current_scene_mut(&mut self) -> &mut Scene
    {
        self.scenes
            .get_mut(&self.current)
            .expect("Current scene set to invalid scene. This should be impossible")
            .resolve()
    }

    pub fn switch(&mut self, target: &'static str) -> Option<&mut Scene>
    {
        let result = self.scenes.get_mut(target)?.resolve();
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
        f(new_scene.resolve());

        true
    }
}
