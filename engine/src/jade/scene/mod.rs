pub mod manager;

use crate::{
    jade::{
        audio::SoundHandler, camera::Camera, ecs::{system::{Scheduler, Stage, System}, world::{Resource, World}}, input::InputState,
    }, util::assets::assetpool::AssetPool,
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

    // Wrap the scheduler functions as they are linked to the world

    pub fn run_stage(&mut self, stage: Stage)
    {
        self.scheduler.run_stage(stage, &mut self.world);
    }

    pub fn add_system<S: System>(&mut self, stage: Stage, system: S)
    {
        self.scheduler.add_system(stage, system);
    }
}
