use std::any::Any;

use crate::{
    jade::{audio::SoundHandler, camera::Camera, ecs::object::Object, input::InputState},
    util::assets::assetpool::AssetPool,
};

pub trait Component: Any
{
    fn start(&mut self, _parent: &mut Object, _ctx: &mut ComponentContext) {}

    fn tick(&mut self, _parent: &mut Object, _ctx: &mut ComponentContext, _dt: f64) {}
}

pub struct ComponentContext<'a>
{
    pub input: &'a InputState,
    pub assetpool: &'a AssetPool,
    pub camera: &'a mut Camera,
    pub sound: &'a mut SoundHandler,
}
