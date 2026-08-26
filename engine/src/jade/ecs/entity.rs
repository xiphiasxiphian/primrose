use std::any::TypeId;

use crate::jade::ecs::{component::Component, world::World};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity
{
    index: u32,
    generation: u32,
}

#[derive(Default, Debug)]
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
            Entity {
                index,
                generation: self.generations[index as usize],
            }
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
        self.generations
            .get(entity.index as usize)
            .map(|&g| g == entity.generation)
            .unwrap_or(false)
    }
}

pub struct EntityBuilder<'a>
{
    world: &'a mut World,
    entity: Entity,
    components: Vec<(TypeId, Box<dyn Component>)>,
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

    pub fn with_component<C: Component>(mut self, component: C) -> Self
    {
        self.components.push((TypeId::of::<C>(), Box::new(component)));
        self
    }

    pub fn build(self) -> Entity
    {
        let type_ids = self.components.iter().map(|&(t, _)| t);

        let arch_index = self.world.find_or_create_archetype(type_ids);
        let arch = self
            .world
            .archetype_mut(arch_index)
            .expect("Invalid arch index acquire from world. Idiot Programmer Detected");
        let row = arch.entities().len();

        arch.add(self.entity, self.components);

        self.world.entity_map_entry(entity).insert_entry((arch_index, row));

        self.entity
    }
}
