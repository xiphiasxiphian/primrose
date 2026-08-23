use crate::jade::ecs::{component::Component, world::World};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity
{
    index: u32,
    generation: u32,
}

pub struct EntityAllocator
{
    generations: Vec<u32>,
    free: Vec<u32>,
}

impl EntityAllocator
{
    pub fn alloc(&mut self) -> Entity
    {
        if let Some(index) = self.free.pop()
        {
            Entity { index, generation: self.generations[index as usize] }
        }
        else
        {
            let index = self.generations.len() as u32;
            self.generations.push(0);

            Entity { index, generation: 0 }
        }
    }

    pub fn dealloc(&mut self, entity: Entity)
    {
        self.generations[entity.index as usize] += 1;
        self.free.push(entity.index);
    }

    pub fn is_alive(&self, entity: Entity) -> bool
    {
        self.generations.get(entity.index as usize)
            .map(|&g| g == entity.generation)
            .unwrap_or(false)
    }
}

pub struct EntityBuilder<'a>
{
    world: &'a mut World,
    entity: Entity,
    components: Vec<&'a dyn Component>,
}

impl<'a> EntityBuilder<'a>
{
    pub fn new(world: &'a mut World, entity: Entity) -> Self
    {
        Self {
            world,
            entity,
            components: vec![],
        }
    }
}
