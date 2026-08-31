use strum::EnumCount;

use crate::jade::ecs::{
    system::{IntoSystem, System, SystemParam},
    world::World,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
pub enum Stage
{
    PreUpdate,
    Update,
    PostUpdate,
    PreRender,
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
    pub fn add_system<Q: SystemParam, S: IntoSystem<Q>>(&mut self, stage: Stage, system: S)
    {
        self.stages[stage as usize].push(Box::new(system.into_system()));
    }

    pub fn run_all(&mut self, world: &mut World)
    {
        for stage in self.order
        {
            let Some(systems) = self.stages.get_mut(stage as usize)
            else
            {
                continue;
            };
            systems.iter_mut().for_each(|x| x.run(world));
        }
    }

    pub fn run_stage(&mut self, stage: Stage, world: &mut World)
    {
        let Some(systems) = self.stages.get_mut(stage as usize)
        else
        {
            return;
        };
        systems.iter_mut().for_each(|x| x.run(world));
    }
}
