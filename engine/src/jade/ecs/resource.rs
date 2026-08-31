use std::any::Any;

pub trait Resource: Any {}
pub use proc_macros::Resource;
