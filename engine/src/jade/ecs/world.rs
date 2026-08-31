use std::{
    any::{Any, TypeId}, cell::RefCell, collections::{HashMap, hash_map::Entry}, rc::Rc,
};

use crate::jade::{ecs::{
    components::{Archetype, Column}, entity::{Entity, EntityAllocator, EntityBuilder}, query::{Query, QueryParam}, resource::Resource,
}, scene::manager::SceneManager};

type EntityLoc = (usize, usize);

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
    pub fn spawn(&mut self) -> EntityBuilder<'_>
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

    pub(super) fn entity_map_entry(&mut self, entity: Entity) -> Entry<'_, Entity, EntityLoc>
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
            .and_then(|r| (r.as_ref() as &dyn Any).downcast_ref::<R>())
    }

    pub fn resource_mut<R: Resource>(&mut self) -> Option<&mut R>
    {
        self.resources
            .get_mut(&TypeId::of::<R>())
            .and_then(|r| (r.as_mut() as &mut dyn Any).downcast_mut::<R>())
    }

    pub fn remove_resource<R: Resource>(&mut self) -> Option<R>
    {
        self.resources
            .remove(&TypeId::of::<R>())
            .and_then(|r| {
                let any_box: Box<dyn Any> = r;
                any_box.downcast::<R>().ok()
            })
            .map(|x| *x)
    }

    pub fn archetypes_iter(&self) -> impl Iterator<Item = &Archetype> { self.archetypes.iter() }

    pub fn query<P: QueryParam>(&self) -> Query<'_, P> { Query::new(self) }

    pub fn find_or_create_archetype(&mut self, type_ids: &[TypeId]) -> usize
    {
        if let Some(index) = self.archetypes.iter().position(|a| a.matches(type_ids))
        {
            return index;
        }

        let columns = type_ids.iter().map(|&tid| (tid, Column::new(tid))).collect();

        self.archetypes
            .push(Archetype::new(type_ids.iter().copied().collect(), vec![], columns));

        self.archetypes.len() - 1
    }

    pub fn archetype_mut(&mut self, index: usize) -> Option<&mut Archetype> { self.archetypes.get_mut(index) }
}
