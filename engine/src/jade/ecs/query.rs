use std::{any::TypeId, marker::PhantomData};

use crate::jade::ecs::{
    components::{Archetype, Component},
    world::World,
};

pub struct Query<'a, T: QueryParam>
{
    world: &'a World,
    _pd: PhantomData<T>,
}

mod private
{
    pub trait Sealed {}
}

pub trait QueryParam: private::Sealed
{
    type Item<'a>;

    fn type_ids() -> Vec<TypeId>;
    fn fetch<'a>(archetype: &'a Archetype, row: usize) -> Option<Self::Item<'a>>;
}

macro_rules! impl_query_param_tuple {
    ($($T:ident),+) => {
        impl<$($T: Component),+> private::Sealed for ($(&$T,)+) {}

        impl<$($T: Component),+> QueryParam for ($(&$T,)+) {
            type Item<'a> = ($(&'a $T,)+);

            fn type_ids() -> Vec<TypeId> {
                vec![$(TypeId::of::<$T>()),+]
            }

            fn fetch<'a>(archetype: &'a Archetype, row: usize) -> Option<Self::Item<'a>> {
                Some((
                    $(
                        archetype.get_entry(&TypeId::of::<$T>(), row)?,
                    )+
                ))
            }
        }
    };
}

impl<A: Component> private::Sealed for &A {}
impl<A: Component> QueryParam for &A {
    type Item<'a> = &'a A;

    fn type_ids() -> Vec<TypeId> { vec![TypeId::of::<A>()] }

    fn fetch<'a>(archetype: &'a Archetype, row: usize) -> Option<Self::Item<'a>> {
        archetype.get_entry(&TypeId::of::<A>(), row)
    }
}

impl_query_param_tuple!(A);
impl_query_param_tuple!(A, B);
impl_query_param_tuple!(A, B, C);
impl_query_param_tuple!(A, B, C, D);

impl<'a, T: QueryParam> Query<'a, T>
{
    pub fn new(world: &'a World) -> Self
    {
        Self {
            world,
            _pd: PhantomData,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = T::Item<'_>>
    {
        let type_ids = T::type_ids();
        self.world
            .archetypes_iter()
            .filter(move |a| a.matches(&type_ids))
            .flat_map(|a| (0..a.entities().len()).flat_map(move |row| T::fetch(a, row)))
    }
}
