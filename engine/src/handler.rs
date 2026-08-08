use crate::{
    jade::scene::{Scene, manager::ManagedScene},
    util::assets::assetpool::AssetPool,
};

pub trait WindowHandler: 'static
{
    fn textures() -> &'static [(&'static str, &'static [u8])]
    where
        Self: Sized;

    fn sounds() -> &'static [(&'static str, &'static [u8])]
    where
        Self: Sized;

    fn scenes(
        &mut self,
        dims: (f32, f32),
        _assetpool: &AssetPool,
    ) -> impl IntoIterator<Item = (&'static str, ManagedScene)>;
    fn initial_scene() -> &'static str;
}
