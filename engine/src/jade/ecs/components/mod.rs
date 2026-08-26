pub mod renderable;
pub mod transform;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::jade::ecs::entity::Entity;

pub trait Component: Any + 'static {}

pub struct Column
{
    data: Vec<Box<dyn Any>>,
    type_id: TypeId,
}

impl Column
{
    pub fn new(tid: TypeId) -> Self
    {
        Self {
            data: Default::default(),
            type_id: tid,
        }
    }
}

#[derive(derive_new::new)]
pub struct Archetype
{
    component_types: Vec<TypeId>,
    entities: Vec<Entity>,
    columns: HashMap<TypeId, Column>,
}

impl Archetype
{
    pub fn entities(&self) -> &[Entity] { &self.entities }

    pub fn matches<'a, I>(&self, types: I) -> bool
    where
        I: IntoIterator<Item = &'a TypeId>,
    {
        types.into_iter().all(|t| self.component_types.contains(t))
    }

    pub fn get_entry<'a, E: 'static>(&'a self, id: &TypeId, row: usize) -> Option<&'a E>
    {
        self.columns.get(id)?.data.get(row)?.downcast_ref::<E>()
    }

    pub fn add(&mut self, entity: Entity, components: impl IntoIterator<Item = (TypeId, Box<dyn Component>)>)
    {
        self.entities.push(entity);

        for (type_id, component) in components
        {
            self.columns
                .get_mut(&type_id)
                .expect("Archetype missing column for type. This should be impossible")
                .data
                .push(component);
        }
    }

    pub fn remove(&mut self, row: usize) -> Entity
    {
        let last = *self.entities.last().unwrap();
        self.entities.swap_remove(row);

        for column in self.columns.values_mut()
        {
            column.data.swap_remove(row);
        }

        last
    }
}
