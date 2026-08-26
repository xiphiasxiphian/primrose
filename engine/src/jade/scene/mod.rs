pub mod manager;

use crate::{
    jade::{
        audio::SoundHandler,
        camera::Camera,
        ecs::{system::Scheduler, world::World},
        input::InputState,
    },
    renderer::Renderable,
    util::assets::assetpool::AssetPool,
};

pub struct Scene
{
    world: World,
    scheduler: Scheduler,
    pub camera: Camera,
}

impl Scene
{
    pub fn new(viewport_dims: (f32, f32)) -> Self
    {
        Self {
            world: World::default(),
            scheduler: Scheduler::default(),
            camera: Camera::new(viewport_dims),
        }
    }
}

// pub struct ComponentContextIn<'a>
// {
//     pub input: &'a InputState,
//     pub assetpool: &'a AssetPool,
//     pub sound: &'a mut SoundHandler,
// }

// impl<'a> ComponentContextIn<'a>
// {
//     pub fn resolve<'b, 'c>(&'c mut self, camera: &'b mut Camera) -> ComponentContext<'b>
//     where
//         'a: 'b,
//         'c: 'b,
//     {
//         ComponentContext {
//             input: self.input,
//             assetpool: self.assetpool,
//             camera,
//             sound: self.sound,
//         }
//     }
// }
