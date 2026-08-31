pub mod scheduler;

use std::marker::PhantomData;

use crate::jade::ecs::{
    query::{Query, QueryParam},
    world::World,
};

pub trait System: 'static
{
    fn run(&mut self, world: &mut World);
}

pub trait SystemParam
{
    type Param<'a>;
    fn fetch<'a>(world: &'a mut World) -> Self::Param<'a>;
}

impl SystemParam for &mut World
{
    type Param<'a> = &'a mut World;
    fn fetch<'a>(world: &'a mut World) -> Self::Param<'a> { world }
}

pub struct FunctionSystem<F, P>
{
    func: F,
    _pd: PhantomData<fn(P)>,
}

impl<F, S> System for FunctionSystem<F, S>
where
    S: SystemParam + 'static,
    F: FnMut(S::Param<'_>) + 'static,
{
    fn run(&mut self, world: &mut World)
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

impl<F, S> IntoSystem<S> for F
where
    S: SystemParam + 'static,
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
