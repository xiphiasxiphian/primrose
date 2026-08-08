use crate::{jade::scene::Scene, util::assets::assetpool::AssetPool};

pub trait WindowHandler: 'static
{
    fn textures() -> &'static [(&'static str, &'static [u8])]
    where Self: Sized;

    fn sounds() -> &'static [(&'static str, &'static [u8])]
    where Self: Sized;

    fn initial_scene(&mut self, dims: (f32, f32), _assetpool: &AssetPool) -> Scene { Scene::new(dims) }
}
