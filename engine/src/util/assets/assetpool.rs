use std::{collections::HashMap, sync::Arc};

use image::ImageError;
use kira::sound::{FromFileError, static_sound::StaticSoundData};
use wgpu::{BindGroupLayout, Device, Queue};

use crate::{jade::audio::Sound, renderer::texture::Texture};

pub type Asset<T> = Arc<T>;
pub type TextureAsset = Asset<Texture>;
pub type SoundAsset = Sound;

#[derive(Default)]
pub struct AssetPool
{
    textures: HashMap<&'static str, TextureAsset>,
    sounds: HashMap<&'static str, SoundAsset>,
}

impl AssetPool
{
    pub fn preloaded(
        textures: &'static [(&'static str, &[u8])],
        sounds: &'static [(&'static str, &[u8])],
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> Result<Self, AssetPoolError>
    {
        let mut pool = Self::default();

        for (name, bytes) in textures
        {
            let texture = Texture::from_bytes(bytes, device, queue, layout)?;
            pool.textures.insert(name, Arc::new(texture));
            log::info!("Loaded texture: {}", name);
        }

        for (name, bytes) in sounds
        {
            let cursor = std::io::Cursor::new(bytes);
            let sound = StaticSoundData::from_cursor(cursor)?;

            pool.sounds.insert(name, sound);
            log::info!("Loaded sound: {}", name);
        }

        Ok(pool)
    }

    pub fn get_texture(&self, id: &'static str) -> Result<TextureAsset, AssetPoolError>
    {
        self.textures
            .get(id)
            .map(Arc::clone)
            .ok_or(AssetPoolError::NotFound(id))
    }

    pub fn get_sound(&self, id: &'static str) -> Result<SoundAsset, AssetPoolError>
    {
        self.sounds.get(id).cloned().ok_or(AssetPoolError::NotFound(id))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AssetPoolError
{
    #[error("Texture '{0}' doesn't exist in pool")]
    NotFound(&'static str),
    #[error("Image decode error: {0}")]
    ImageError(#[from] ImageError),
    #[error("Sound decode error: {0}")]
    SoundError(#[from] FromFileError),
}
