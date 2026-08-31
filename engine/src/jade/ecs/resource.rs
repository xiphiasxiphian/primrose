use std::{any::{Any, TypeId}, cell::{Ref, RefCell, RefMut}, rc::Rc};

pub trait Resource: Any
{
}
pub use proc_macros::Resource;
