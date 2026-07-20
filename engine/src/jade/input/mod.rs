pub mod key;
pub mod mouse;

use std::{cell::RefCell, rc::Rc};

use log::info;
use strum::EnumCount;
use wasm_bindgen::{JsCast, convert::FromWasmAbi, prelude::Closure};
use web_sys::{Event, EventTarget, KeyboardEvent, MouseEvent, Window};

use crate::jade::input::{key::Key, mouse::MouseButton};

#[derive(Debug)]
pub struct InputState
{
    keys_held: [bool; Key::COUNT],
    keys_down: [bool; Key::COUNT],
    keys_up: [bool; Key::COUNT],
    mouse_pos: (i32, i32),
    mouse_delta: (i32, i32),
    mouse_buttons: [bool; MouseButton::COUNT],
    mouse_down: [bool; MouseButton::COUNT],
    mouse_up: [bool; MouseButton::COUNT],
}

impl InputState
{
    pub fn new() -> Self
    {
        Self {
            keys_held: [false; Key::COUNT],
            keys_down: [false; Key::COUNT],
            keys_up: [false; Key::COUNT],
            mouse_pos: (0, 0),
            mouse_delta: (0, 0),
            mouse_buttons: [false; MouseButton::COUNT],
            mouse_down: [false; MouseButton::COUNT],
            mouse_up: [false; MouseButton::COUNT],
        }
    }

    pub fn flush(&mut self)
    {
        self.mouse_delta = (0, 0);

        self.keys_up = [false; Key::COUNT];
        self.keys_down = [false; Key::COUNT];
        self.mouse_down = [false; MouseButton::COUNT];
        self.mouse_up = [false; MouseButton::COUNT];
    }

    pub fn attach_listeners(state: Rc<RefCell<Self>>, window: &mut Window, target: &EventTarget)
    {
        // keydown listener
        Self::attach_listener(
            state.clone(),
            &window,
            |st| {
                move |e: KeyboardEvent| {
                    // ignore any unknown keys
                    if let Ok(key) = Key::try_from(e.code().as_str())
                    {
                        let mut s = st.borrow_mut();
                        s.keys_down[key as usize] = true;
                        s.keys_held[key as usize] = true;
                    }
                }
            },
            "keydown",
        );

        // keyup listener
        Self::attach_listener(
            state.clone(),
            &window,
            |st| {
                move |e: KeyboardEvent| {
                    if let Ok(key) = Key::try_from(e.code().as_str())
                    {
                        let mut s = st.borrow_mut();
                        s.keys_up[key as usize] = true;
                        s.keys_held[key as usize] = false;
                    }
                }
            },
            "keyup",
        );

        // mousemove
        Self::attach_listener(
            state.clone(),
            &window,
            |st| {
                move |e: MouseEvent| {
                    let mut s = st.borrow_mut();
                    s.mouse_delta = (e.movement_x(), e.movement_y());
                    s.mouse_pos = (e.offset_x(), e.offset_y());
                }
            },
            "mousemove",
        );

        // mousedown
        Self::attach_listener(
            state.clone(),
            &target,
            |st| {
                move |e: MouseEvent| {
                    // button: 0=left, 1=middle, 2=right

                    if let Ok(button) = MouseButton::try_from(e.button())
                    {
                        let mut s = st.borrow_mut();
                        let index = button as usize;
                        s.mouse_buttons[index] = true;
                        s.mouse_down[index] = true;
                    }
                }
            },
            "mousedown",
        );

        // mouseup
        Self::attach_listener(
            state.clone(),
            &target,
            |st| {
                move |e: MouseEvent| {
                    // button: 0=left, 1=middle, 2=right

                    if let Ok(button) = MouseButton::try_from(e.button())
                    {
                        let mut s = st.borrow_mut();
                        let index = button as usize;
                        s.mouse_buttons[index] = false;
                        s.mouse_up[index] = true;
                    }
                }
            },
            "mouseup",
        );
    }

    fn attach_listener<A, T, F, F2>(state: Rc<RefCell<Self>>, attachment_target: A, callback: F, listener: &str)
    where
        T: FromWasmAbi + AsRef<Event>,
        F2: FnMut(T) + 'static,
        F: FnOnce(Rc<RefCell<Self>>) -> F2,
        A: AsRef<EventTarget>,
    {
        let cb = Closure::<dyn FnMut(T)>::new(callback(state));

        attachment_target
            .as_ref()
            .add_event_listener_with_callback(listener, cb.as_ref().unchecked_ref())
            .expect(format!("Failed to attach \'{}\' listener", listener).as_ref());

        info!("Attached \'{}\' listener", listener);
        cb.forget()
    }

    pub fn is_key_down(&self, key: Key) -> bool { self.keys_down[key as usize] }

    pub fn is_key_up(&self, key: Key) -> bool { self.keys_up[key as usize] }

    pub fn is_key_held(&self, key: Key) -> bool { self.keys_held[key as usize] }

    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool { self.mouse_down[button as usize] }

    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool { self.mouse_up[button as usize] }

    pub fn mouse_pos(&self) -> (i32, i32) { self.mouse_pos }

    pub fn mouse_delta(&self) -> (i32, i32) { self.mouse_delta }
}
