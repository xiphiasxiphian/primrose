pub mod scheduler;

use std::marker::PhantomData;

use crate::jade::ecs::{
    query::QueryParam,
    world::World,
};

pub trait System<'w>: 'static
{
    fn run(&mut self, world: &mut World<'w>);
}

pub trait SystemParam<'w>
{
    type Param<'a> where 'w: 'a;
    fn fetch<'a>(world: &'a mut World<'w>) -> Self::Param<'a>;
}

impl<'w> SystemParam<'w> for &mut World<'w>
{
    type Param<'a> = &'a mut World<'w>;
    fn fetch<'a>(world: &'a mut World<'w>) -> Self::Param<'a> { world }
}

pub struct FunctionSystem<F, P>
{
    func: F,
    _pd: PhantomData<fn(P)>,
}

impl<'w, F, S> System<'w> for FunctionSystem<F, S>
where
    S: SystemParam<'w> + 'static,
    F: FnMut(S::Param<'_>) + 'static,
{
    fn run(&mut self, world: &mut World<'w>)
    {
        let param = S::fetch(world);
        (self.func)(param)
    }
}

pub trait IntoSystem<Q>
{
    type System: System;
    fn into_system(self) -> Self::System;
}

impl<'w, F, S> IntoSystem<S> for F
where
    S: SystemParam<'w> + 'static,
    F: FnMut(S::Param<'_>) + 'static,
{
    type System = FunctionSystem<F, S>;

    fn into_system(self) -> Self::System
    {
        FunctionSystem {
            func: self,
            _pd: PhantomData,
        }
    }
}
