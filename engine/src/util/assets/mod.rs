use wgpu::{ShaderModuleDescriptor, include_wgsl};

pub mod assetpool;

pub const TEXTURES: &[(&'static str, &'static [u8])] = &[("grass", include_bytes!("../../../assets/images/grass.png"))];

pub const SHADERS: &[(&'static str, ShaderModuleDescriptor)] =
    &[("main", include_wgsl!("../../../assets/shaders/shader.wgsl"))];

pub const SOUNDS: &[(&'static str, &[u8])] = &[];
