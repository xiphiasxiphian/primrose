use winit::{dpi::LogicalSize, window::{Fullscreen, WindowAttributes}};

use crate::jade::input::key::Key;

#[derive(Clone, Copy, Debug)]
pub struct WindowDescriptor
{
    pub title: &'static str,
    pub dims: (u32, u32),
    pub fullscreen_options: Option<FullscreenOptions>,
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
}

impl From<WindowDescriptor> for WindowAttributes
{
    fn from(descriptor: WindowDescriptor) -> Self
    {
        WindowAttributes::default()
            .with_title(descriptor.title)
            .with_inner_size(LogicalSize::new(descriptor.dims.0, descriptor.dims.1))
            .with_fullscreen(descriptor.get_fullscreen())
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
