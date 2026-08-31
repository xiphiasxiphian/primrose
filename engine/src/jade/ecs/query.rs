use std::{any::TypeId, collections::HashSet, marker::PhantomData};

use crate::jade::ecs::{
    components::{Archetype, Component},
    system::SystemParam,
    world::World,
};

pub struct Query<'a, T: QueryParam>
{
    world: &'a World,
    _pd: PhantomData<T>,
}

impl<Q: QueryParam> SystemParam for Query<'_, Q>
{
    type Param<'a> = Query<'a, Q>;
    fn fetch<'a>(world: &'a mut World) -> Self::Param<'a> { world.query::<Q>() }
}

mod private
{
    pub trait Sealed {}
}

pub trait QueryParam: private::Sealed
{
    type Item<'a>;

    fn type_ids() -> Vec<(TypeId, bool)>;
    unsafe fn fetch<'a>(archetype: &'a Archetype, row: usize) -> Option<Self::Item<'a>>;
}

impl<A: Component> private::Sealed for &A {}
impl<A: Component> QueryParam for &A
{
    type Item<'a> = &'a A;

    fn type_ids() -> Vec<(TypeId, bool)> { vec![(TypeId::of::<A>(), false)] }

    unsafe fn fetch<'a>(archetype: &'a Archetype, row: usize) -> Option<Self::Item<'a>>
    {
        // SAFETY:
        // - Shared Immutable References are safe by default
        archetype.get_entry::<A>(row)
    }
}

impl<A: Component> private::Sealed for &mut A {}
impl<A: Component> QueryParam for &mut A
{
    type Item<'a> = &'a mut A;

    fn type_ids() -> Vec<(TypeId, bool)> { vec![(TypeId::of::<A>(), true)] }

    unsafe fn fetch<'a>(archetype: &'a Archetype, row: usize) -> Option<Self::Item<'a>>
    {
        // SAFETY:
        // - Each TypeId maps to exactly one column (one Vec)
        // - The query construction verified no TypeId appears twice as mutable
        // - Therefore this &mut T cannot alias any other &mut produced by this query
        unsafe { archetype.get_entry_mut::<A>(row) }
    }
}

fn validate_query_params(type_ids: &[(TypeId, bool)])
{
    let mut seen_mutable = HashSet::new();
    let mut seen_any = HashSet::new();

    for (tid, is_mut) in type_ids
    {
        assert!(
            !(*is_mut && seen_any.contains(tid)),
            "Query: mutable access to {:?} conflicts with existing immutable access",
            tid
        );

        assert!(
            !seen_mutable.contains(tid),
            "Query: duplicate mutable access to {:?}",
            tid,
        );

        if *is_mut
        {
            seen_mutable.insert(*tid);
        }
        seen_any.insert(*tid);
    }
}

pub struct QueryIter<'a, Q: QueryParam, I: Iterator<Item = &'a Archetype>>
{
    archetypes: I,
    current: Option<&'a Archetype>,
    row: usize,
    _pd: PhantomData<Q>,
}

impl<'a, Q: QueryParam, I: Iterator<Item = &'a Archetype>> Iterator for QueryIter<'a, Q, I>
{
    type Item = Q::Item<'a>;

    fn next(&mut self) -> Option<Self::Item>
    {
        loop
        {
            if let Some(arch) = self.current
                && self.row < arch.entities().len()
            {
                let row = self.row;
                self.row += 1;

                // SAFETY: validate_query_params already has confirmed no aliasing
                return Some(unsafe { Q::fetch(arch, row)? });
            }

            loop
            {
                let arch = self.archetypes.next()?;
                if arch.matches(Q::type_ids().iter().map(|(t, _)| t))
                {
                    self.current = Some(arch);
                    self.row = 0;
                    break;
                }
            }
        }
    }
}

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
        validate_query_params(&type_ids);

        QueryIter {
            archetypes: self.world.archetypes_iter(),
            current: None,
            row: 0,
            _pd: PhantomData::<T>,
        }
    }
}

macro_rules! impl_query_param_tuple {
    ($F:ident $(, $rest:ident)*) => {
        impl<$F: QueryParam, $($rest: QueryParam),*> private::Sealed for ($F, $($rest),*) {}
        impl<$F: QueryParam, $($rest: QueryParam),*> QueryParam for ($F, $($rest),*) {
            type Item<'a> = ($F::Item<'a>, $($rest::Item<'a>),*);

            fn type_ids() -> Vec<(TypeId, bool)> {
                let mut ids = Vec::new();
                ids.extend($F::type_ids());
                $(ids.extend($rest::type_ids());)*
                ids
            }

            unsafe fn fetch<'a>(arch: &'a Archetype, row: usize) -> Option<Self::Item<'a>> {
                unsafe {
                    Some((
                        $F::fetch(arch, row)?,
                        $($rest::fetch(arch, row)?),*
                    ))
                }
            }
        }

        impl_query_param_tuple!($($rest),*);
    };

    () => {};
}

impl_query_param_tuple!(A, B, C, D, E, F, G, H);
