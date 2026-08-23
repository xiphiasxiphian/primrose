use std::{any::{Any, TypeId}, collections::HashMap};

use crate::jade::ecs::{component::Archetype, entity::{Entity, EntityAllocator, EntityBuilder}};

type EntityLoc = (usize, usize);

pub trait Resource: Any {}

pub struct World
{
    entities: EntityAllocator,
    archetypes: Vec<Archetype>,
    entity_map: HashMap<Entity, EntityLoc>,
    resources: HashMap<TypeId, Box<dyn Resource>>,
}

impl World
{
    pub fn spawn(&mut self) -> EntityBuilder
    {
        let entity = self.entities.alloc();
        EntityBuilder::new(self, entity)
    }

    pub fn despawn(
        &mut self,
        entity: Entity,
    )
    {
        if let Some((arch_index, row)) = self.entity_map.remove(&entity)
        {
            let arch = &mut self.archetypes[arch_index];

            let last = arch.remove(row);
            if last != entity
            {
                self.entity_map.insert(last, (arch_index, row));
            }
        }

        self.entities.dealloc(entity);
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R)
    {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }

    pub fn resource<R: Resource>(&self) -> Option<&R>
    {
        self.resources.get(&TypeId::of::<R>())
            .and_then(|r| (r as &dyn Any).downcast_ref::<R>())
    }

    pub fn resource_mut<R: Resource>(&mut self) -> Option<&mut R>
    {
        self.resources.get_mut(&TypeId::of::<R>())
            .and_then(|r| (r as &mut dyn Any).downcast_mut::<R>())
    }

    pub fn archetypes_iter(&self) -> impl Iterator<Item = &Archetype>
    {
        self.archetypes.iter()
    }
}
