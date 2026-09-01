use color_eyre::eyre;
use proc_macros::WithBuilder;
use winit::{dpi::LogicalSize, window::{Fullscreen, Icon, WindowAttributes}};

use crate::jade::input::key::Key;

#[derive(Clone, Copy, Debug, WithBuilder)]
pub struct WindowDescriptor
{
    pub title: &'static str,
    pub dims: (u32, u32),
    pub fullscreen_options: Option<FullscreenOptions>,
    pub icon: Option<&'static str>,
}

impl Default for WindowDescriptor
{
    fn default() -> Self
    {
        let dims = (1440, 810);
        Self {
            title: "Default Title",
            dims,
            fullscreen_options: Some(FullscreenOptions::default()),
            icon: None,
        }
    }
}

impl WindowDescriptor
{
    pub fn get_fullscreen(&self) -> Option<Fullscreen>
    {
        self.fullscreen_options
            .and_then(|x| x.on_start.then_some(Fullscreen::Borderless(None)))
    }

    pub fn load_icon(path: &str) -> eyre::Result<Icon>
    {
        let bytes = std::fs::read(path)?;

        let (rgba, width, height) = {
            let image = image::load_from_memory(&bytes)?.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        Ok(Icon::from_rgba(rgba, width, height)?)
    }

    pub fn get_icon(&self) -> Option<Icon>
    {
        self.icon.and_then(|path| WindowDescriptor::load_icon(path).map_or_else(|x| {
                log::warn!("Failed to load icon at {}: {}", self.icon.unwrap_or("{unknown}"), x);
                None
            }, |x| Some(x)))
    }
}

impl From<WindowDescriptor> for WindowAttributes
{
    fn from(descriptor: WindowDescriptor) -> Self
    {
        WindowAttributes::default()
            .with_title(descriptor.title)
            .with_inner_size(LogicalSize::new(descriptor.dims.0, descriptor.dims.1))
            .with_fullscreen(descriptor.get_fullscreen())
            .with_window_icon(descriptor.get_icon())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FullscreenOptions
{
    pub on_start: bool,
    pub toggle_key: Key,
}

impl Default for FullscreenOptions
{
    fn default() -> Self
    {
        Self {
            on_start: false,
            toggle_key: Key::F11,
        }
    }
}

impl FullscreenOptions
{
    pub const DEFAULT_ESCAPE_KEY: Key = Key::Escape;
}
