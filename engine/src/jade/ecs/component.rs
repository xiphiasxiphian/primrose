use std::{any::{Any, TypeId}, collections::HashMap};

use crate::jade::ecs::entity::Entity;

pub trait Component: Any + 'static {}

struct Column
{
    data: Vec<Box<dyn Any>>,
    type_id: TypeId,
}

pub struct Archetype
{
    component_types: Vec<TypeId>,
    entities: Vec<Entity>,
    columns: HashMap<TypeId, Column>,
}

impl Archetype
{
    pub fn entities(&self) -> &[Entity] { &self.entities }

    pub fn matches(&self, types: &[TypeId]) -> bool
    {
        types.iter().all(|t| self.component_types.contains(t))
    }

    pub fn get_entry<'a, E: 'static>(&'a self, id: &TypeId, row: usize) -> Option<&'a E>
    {
        self.columns.get(id)?.data.get(row)?.downcast_ref::<E>()
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
