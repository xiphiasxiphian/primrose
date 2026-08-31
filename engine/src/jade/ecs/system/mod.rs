pub mod scheduler;

use std::marker::PhantomData;

use crate::jade::{ecs::world::World, scene::manager::GlobalResources};

pub trait System: 'static
{
    fn run(&mut self, world: &mut World, globals: &mut GlobalResources);
}

pub trait SystemParam
{
    type Param<'a>;
    fn fetch<'a>(world: &'a mut World) -> Self::Param<'a>;
}

impl SystemParam for &'static mut World
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
    F: FnMut(S::Param<'_>, &mut GlobalResources) + 'static,
{
    fn run(&mut self, world: &mut World, globals: &mut GlobalResources)
    {
        let param = S::fetch(world);
        (self.func)(param, globals)
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
    F: for<'a, 'b> FnMut(S::Param<'a>, &'b mut GlobalResources) + 'static,
    F: for<'b> FnMut(S, &'b mut GlobalResources),
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
