use std::{any::TypeId, marker::PhantomData};

use crate::jade::ecs::{component::{Archetype, Component}, world::World};

pub struct Query<'a, T: QueryParam>
{
    world: &'a World,
    _pd: PhantomData<T>,
}

mod private {
    pub trait Sealed {}
}

pub trait QueryParam: private::Sealed
{
    type Item<'a>;

    fn type_ids() -> Vec<TypeId>;
    fn fetch<'a>(archetype: &'a Archetype, row: usize) -> Option<Self::Item<'a>>;
}

impl <A: Component, B: Component> private::Sealed for (&A, &B) {}
impl <A: Component, B: Component> QueryParam for (&A, &B)
{
    type Item<'a> = (&'a A, &'a B);

    fn type_ids() -> Vec<TypeId>
    {
        vec![TypeId::of::<A>(), TypeId::of::<B>()]
    }

    fn fetch<'a>(archetype: &'a Archetype, row: usize) -> Option<Self::Item<'a>>
    {
        let a = archetype.get_entry(&TypeId::of::<A>(), row)?;
        let b = archetype.get_entry(&TypeId::of::<B>(), row)?;

        Some((a, b))
    }
}

impl <'a, T: QueryParam> Query<'a, T>
{
    pub fn iter(&self) -> impl Iterator<Item = T::Item<'_>>
    {
        let type_ids = T::type_ids();
        self.world.archetypes_iter()
            .filter(move |a| a.matches(&type_ids))
            .flat_map(|a| {
                (0..a.entities().len()).flat_map(move |row| T::fetch(a, row))
            })
    }
}
