use std::{
    any::{Any, TypeId},
    collections::{HashMap, hash_map::Entry},
};

use crate::jade::ecs::{
    component::{Archetype, Column},
    entity::{Entity, EntityAllocator, EntityBuilder},
    query::{Query, QueryParam},
};

type EntityLoc = (usize, usize);

pub trait Resource: Any {}

#[derive(Default)]
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

    pub fn despawn(&mut self, entity: Entity)
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

    pub(super) fn entity_map_entry(&mut self, entity: Entity) -> Entry<Entity, EntityLoc>
    {
        self.entity_map.entry(entity)
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R)
    {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }

    pub fn resource<R: Resource>(&self) -> Option<&R>
    {
        self.resources
            .get(&TypeId::of::<R>())
            .and_then(|r| (r as &dyn Any).downcast_ref::<R>())
    }

    pub fn resource_mut<R: Resource>(&mut self) -> Option<&mut R>
    {
        self.resources
            .get_mut(&TypeId::of::<R>())
            .and_then(|r| (r as &mut dyn Any).downcast_mut::<R>())
    }

    pub fn archetypes_iter(&self) -> impl Iterator<Item = &Archetype> { self.archetypes.iter() }

    pub fn query<P: QueryParam>(&self) -> Query<P> { Query::new(self) }

    pub fn find_or_create_archetype<'a>(&mut self, type_ids: impl IntoIterator<Item = &'a TypeId>) -> usize
    {
        if let Some(index) = self.archetypes.iter().position(|a| a.matches(type_ids.into_iter()))
        {
            return index;
        }

        let columns = type_ids.into_iter().map(|&tid| (tid, Column::new(tid))).collect();

        self.archetypes
            .push(Archetype::new(type_ids.into_iter.collect(), vec![], columns));

        self.archetypes.len() - 1
    }

    pub fn archetype_mut(&mut self, index: usize) -> Option<&mut Archetype> { self.archetypes.get_mut(index) }
}
