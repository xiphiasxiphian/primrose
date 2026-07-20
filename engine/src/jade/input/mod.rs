pub mod key;
pub mod mouse;

use std::{cell::RefCell, rc::Rc};

use log::info;
use strum::EnumCount;
use winit::{
    dpi::{PhysicalPosition, Position},
    event::{ElementState, KeyEvent},
    keyboard::PhysicalKey,
};

use crate::jade::input::{key::Key, mouse::MouseButton};

pub type MousePos = PhysicalPosition<f64>;

#[derive(Debug)]
pub struct InputState
{
    keys_held: [bool; Key::COUNT],
    keys_down: [bool; Key::COUNT],
    keys_up: [bool; Key::COUNT],
    mouse_pos: MousePos,
    mouse_delta: MousePos,
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
            mouse_pos: MousePos::default(),
            mouse_delta: MousePos::default(),
            mouse_buttons: [false; MouseButton::COUNT],
            mouse_down: [false; MouseButton::COUNT],
            mouse_up: [false; MouseButton::COUNT],
        }
    }

    pub fn flush(&mut self)
    {
        self.mouse_delta = MousePos::default();

        self.keys_up = [false; Key::COUNT];
        self.keys_down = [false; Key::COUNT];
        self.mouse_down = [false; MouseButton::COUNT];
        self.mouse_up = [false; MouseButton::COUNT];
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent)
    {
        let PhysicalKey::Code(code) = key_event.physical_key
        else
        {
            return;
        };
        let Ok(key) = Key::try_from(code)
        else
        {
            return;
        };

        match key_event.state
        {
            ElementState::Pressed =>
            {
                self.keys_down[key as usize] = true;
                self.keys_held[key as usize] = true;
            }
            ElementState::Released =>
            {
                self.keys_up[key as usize] = true;
                self.keys_held[key as usize] = false;
            }
        }
    }

    pub fn handle_cursor_event(&mut self, position: PhysicalPosition<f64>) { self.mouse_pos = position; }

    pub fn handle_mouse_event(&mut self, state: ElementState, button: winit::event::MouseButton)
    {
        let Ok(btn) = MouseButton::try_from(button)
        else
        {
            return;
        };

        match state
        {
            ElementState::Pressed => self.mouse_buttons[btn as usize] = true,
            ElementState::Released => self.mouse_buttons[btn as usize] = false,
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool { self.keys_down[key as usize] }

    pub fn is_key_up(&self, key: Key) -> bool { self.keys_up[key as usize] }

    pub fn is_key_held(&self, key: Key) -> bool { self.keys_held[key as usize] }

    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool { self.mouse_down[button as usize] }

    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool { self.mouse_up[button as usize] }

    pub fn mouse_pos(&self) -> MousePos { self.mouse_pos }

    pub fn mouse_delta(&self) -> MousePos { self.mouse_delta }
}
