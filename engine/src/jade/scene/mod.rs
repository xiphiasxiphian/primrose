pub mod manager;

use std::{cell::RefCell, rc::Rc};

use crate::jade::{
    camera::Camera,
    ecs::{
        entity::EntityBuilder,
        system::{Scheduler, Stage, System},
        world::{Resource, World},
    },
    input::InputState,
};

pub struct Scene
{
    pub world: World,
    scheduler: Scheduler,
    pub camera: Camera,
}

impl Scene
{
    pub fn new(viewport_dims: (f32, f32)) -> Self
    {
        Self {
            world: World::default(),
            scheduler: Scheduler::default(),
            camera: Camera::new(viewport_dims),
        }
    }

    pub fn with_resource<R: Resource>(mut self, resource: R) -> Self
    {
        self.world.insert_resource(resource);
        self
    }

    pub fn with_entity<F>(mut self, f: F) -> Self
    where
        F: FnOnce(EntityBuilder<'_>) -> EntityBuilder<'_>,
    {
        f(self.world.spawn()).build();
        self
    }

    // Wrap the scheduler functions as they are linked to the world

    pub fn run_stage(&mut self, stage: Stage) { self.scheduler.run_stage(stage, &mut self.world); }

    pub fn with_system<S: System>(mut self, stage: Stage, system: S) -> Self
    {
        self.scheduler.add_system(stage, system);
        self
    }

    pub fn add_system<S: System>(&mut self, stage: Stage, system: S) { self.scheduler.add_system(stage, system); }
}

pub struct EngineState
{
    pub input: Rc<RefCell<InputState>>,
}
