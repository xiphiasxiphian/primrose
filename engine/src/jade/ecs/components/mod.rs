pub mod basic_controller;
pub mod camera
{
    pub mod camera_lock;
}

macro_rules! default_any_impl {
    () => {
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    };
}

pub(super) use default_any_impl;
