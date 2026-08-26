use std::collections::HashMap;

use strum::EnumCount;

use crate::jade::ecs::world::World;

pub trait System: 'static
{
    fn run(&mut self, world: &mut World);
}

impl<F> System for F
where
    F: Fn(&mut World) + 'static
{
    fn run(&mut self, world: &mut World)
    {
        (self)(world)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
pub enum Stage
{
    PreUpdate,
    Update,
    PostUpdate,
    PreRender
}

pub struct Scheduler
{
    stages: [Vec<Box<dyn System>>; Stage::COUNT],
    order: [Stage; Stage::COUNT],
}

impl Default for Scheduler
{
    fn default() -> Self
    {
        Self {
            stages: Default::default(),
            order: [Stage::PreUpdate, Stage::Update, Stage::PostUpdate, Stage::PreRender],
        }
    }
}

impl Scheduler
{
    pub fn add_system<S: System>(&mut self, stage: Stage, system: S)
    {
        self.stages[stage as usize].push(Box::new(system));
    }

    pub fn run(&mut self, world: &mut World)
    {
        for stage in self.order
        {
            let Some(systems) = self.stages.get_mut(stage as usize) else { continue };
            systems.iter_mut().for_each(|x| x.run(world));
        }
    }
}
