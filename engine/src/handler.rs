use crate::{
    jade::scene::manager::ManagedScene,
    util::assets::{ManagedResource, assetpool::AssetPool},
};

pub trait WindowHandler: 'static
{
    fn textures() -> impl IntoIterator<Item = (&'static str, ManagedResource<&'static [u8]>)>
    where
        Self: Sized;

    fn sounds() -> impl IntoIterator<Item = (&'static str, ManagedResource<&'static [u8]>)>
    where
        Self: Sized;

    fn scenes(
        &mut self,
        dims: (f32, f32),
        _assetpool: &mut AssetPool,
    ) -> impl IntoIterator<Item = (&'static str, ManagedScene)>;
    fn initial_scene() -> &'static str;
}
