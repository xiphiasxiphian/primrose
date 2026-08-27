pub mod renderable;
pub mod transform;

use std::{
    any::{Any, TypeId}, cell::UnsafeCell, collections::HashMap,
};

use crate::jade::ecs::entity::Entity;

pub trait Component: Any + 'static {}
pub use proc_macros::Component;

pub struct Column
{
    data: Vec<UnsafeCell<Box<dyn Any>>>,
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

    pub fn push<T: Component>(&mut self, value: T) {
        self.data.push(UnsafeCell::new(Box::new(value)));
    }

    pub fn get<T: Component>(&self, row: usize) -> Option<&T> {
        // SAFETY: shared reference, no mutation possible through &.
        let inner = unsafe { &*self.data[row].get() };
        inner.downcast_ref::<T>()
    }

    /// SAFETY: caller must guarantee no other
    /// reference to this (type, row) pair exists simultaneously
    pub unsafe fn get_mut<T: Component>(&self, row: usize) -> Option<&mut T>
    {
        // SAFETY: UnsafeCell::get() gives *mut, which we dereference to &mut.
        // The caller upholds the invariant that no aliasing reference exists.
        // This is sound because UnsafeCell opts out of the &-is-immutable rule.
        let inner = unsafe { &mut *self.data[row].get() };
        inner.downcast_mut::<T>()
    }

    pub fn swap_remove(&mut self, row: usize)
    {
        self.data.swap_remove(row);
    }

    pub fn len(&self) -> usize { self.data.len() }
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

    pub fn get_entry<'a, E: Component + 'static>(&'a self, row: usize) -> Option<&'a E>
    {
        self.columns.get(&TypeId::of::<E>())?.get::<E>(row)
    }

    /// SAFETY: caller must guarantee:
    /// 1. No other reference to this component type+row exists
    /// 2. TypeId does not appear twice as mutable in the same query
    /// Both are enforced by validate_query_params / assert_all_disjoint
    pub unsafe fn get_entry_mut<'a, E: Component + 'static>(&'a self, row: usize) -> Option<&'a mut E>
    {
        unsafe { self.columns.get(&TypeId::of::<E>())?.get_mut::<E>(row) }
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
                .push(UnsafeCell::new(component));
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
