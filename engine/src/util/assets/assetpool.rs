use std::{collections::HashMap, sync::Arc};

use image::ImageError;
use kira::sound::{FromFileError, static_sound::StaticSoundData};
use wgpu::{BindGroupLayout, Device, Queue};

use crate::{jade::audio::Sound, renderer::texture::Texture, util::assets::ManagedResource};

pub type Asset<T> = Arc<T>;
pub type TextureAsset = Asset<Texture>;
pub type SoundAsset = Sound;

#[derive(Default)]
pub struct AssetPool
{
    textures: HashMap<&'static str, ManagedResource<TextureAsset>>,
    sounds: HashMap<&'static str, ManagedResource<SoundAsset>>,
}

impl AssetPool
{
    pub fn preloaded(
        textures: impl IntoIterator<Item = (&'static str, ManagedResource<&'static [u8]>)>,
        sounds: impl IntoIterator<Item = (&'static str, ManagedResource<&'static [u8]>)>,
        device: Device,
        queue: Queue,
        layout: BindGroupLayout,
    ) -> Result<Self, AssetPoolError>
    {
        let mut pool = Self::default();

        let default_texture = Arc::new(Texture::from_bytes(
            include_bytes!("../../../assets/images/default.png"),
            &device,
            &queue,
            &layout,
        )?);
        let default_sound = StaticSoundData::from_cursor(std::io::Cursor::new(include_bytes!(
            "../../../assets/sounds/default.wav"
        )))?;

        for (name, bytes_resource) in textures
        {
            let device_clone = device.clone();
            let queue_clone = queue.clone();
            let layout_clone = layout.clone();
            let default_tex_clone = default_texture.clone();

            let texture = bytes_resource.map(move |raw_bytes| {
                Texture::from_bytes(raw_bytes, &device_clone, &queue_clone, &layout_clone).map_or_else(
                    |e| {
                        log::error!("Failed to load texture {}. Raw Error: {}", name, e);
                        default_tex_clone
                    },
                    Arc::new,
                )
            });

            pool.textures.insert(name, texture);
            log::info!("Queued texture: {}", name);
        }

        for (name, bytes_resource) in sounds
        {
            let default_snd_clone = default_sound.clone();

            let sound = bytes_resource.map(move |raw_bytes| {
                let cursor = std::io::Cursor::new(raw_bytes);
                StaticSoundData::from_cursor(cursor).unwrap_or_else(|e| {
                    log::error!("Failed to load sound {}. Raw Error: {}", name, e);
                    default_snd_clone
                })
            });

            pool.sounds.insert(name, sound);
            log::info!("Queued sound: {}", name);
        }

        Ok(pool)
    }

    pub fn get_texture(&mut self, id: &'static str) -> Result<TextureAsset, AssetPoolError>
    {
        self.textures
            .get_mut(id)
            .map(|x| x.get().clone())
            .ok_or(AssetPoolError::NotFound(id))
    }

    pub fn get_sound(&mut self, id: &'static str) -> Result<SoundAsset, AssetPoolError>
    {
        self.sounds
            .get_mut(id)
            .map(|x| x.get().clone())
            .ok_or(AssetPoolError::NotFound(id))
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
