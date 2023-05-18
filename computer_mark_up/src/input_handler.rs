use winit::event::{WindowEvent, VirtualKeyCode, KeyboardInput};
use hashbrown::HashMap;

// Every single key on the keyboard that is handled by wgpu
pub(crate) const VIRTUALKEYS: [VirtualKeyCode; 134] = [
    VirtualKeyCode::Key1,
    VirtualKeyCode::Key2,
    VirtualKeyCode::Key3,
    VirtualKeyCode::Key4,
    VirtualKeyCode::Key5,
    VirtualKeyCode::Key6,
    VirtualKeyCode::Key7,
    VirtualKeyCode::Key8,
    VirtualKeyCode::Key9,
    VirtualKeyCode::Key0,
    VirtualKeyCode::A,
    VirtualKeyCode::B,
    VirtualKeyCode::C,
    VirtualKeyCode::D,
    VirtualKeyCode::E,
    VirtualKeyCode::F,
    VirtualKeyCode::G,
    VirtualKeyCode::H,
    VirtualKeyCode::I,
    VirtualKeyCode::J,
    VirtualKeyCode::K,
    VirtualKeyCode::L,
    VirtualKeyCode::M,
    VirtualKeyCode::N,
    VirtualKeyCode::O,
    VirtualKeyCode::P,
    VirtualKeyCode::Q,
    VirtualKeyCode::R,
    VirtualKeyCode::S,
    VirtualKeyCode::T,
    VirtualKeyCode::U,
    VirtualKeyCode::V,
    VirtualKeyCode::W,
    VirtualKeyCode::X,
    VirtualKeyCode::Y,
    VirtualKeyCode::Z,
    VirtualKeyCode::Escape,
    VirtualKeyCode::F1,
    VirtualKeyCode::F2,
    VirtualKeyCode::F3,
    VirtualKeyCode::F4,
    VirtualKeyCode::F5,
    VirtualKeyCode::F6,
    VirtualKeyCode::F7,
    VirtualKeyCode::F8,
    VirtualKeyCode::F9,
    VirtualKeyCode::F10,
    VirtualKeyCode::F11,
    VirtualKeyCode::F12,
    VirtualKeyCode::F13,
    VirtualKeyCode::F14,
    VirtualKeyCode::F15,
    VirtualKeyCode::Snapshot,
    VirtualKeyCode::Scroll,
    VirtualKeyCode::Pause,
    VirtualKeyCode::Insert,
    VirtualKeyCode::Home,
    VirtualKeyCode::Delete,
    VirtualKeyCode::End,
    VirtualKeyCode::PageDown,
    VirtualKeyCode::PageUp,
    VirtualKeyCode::Left,
    VirtualKeyCode::Up,
    VirtualKeyCode::Right,
    VirtualKeyCode::Down,
    VirtualKeyCode::Back,
    VirtualKeyCode::Return,
    VirtualKeyCode::Space,
    VirtualKeyCode::Numlock,
    VirtualKeyCode::Numpad0,
    VirtualKeyCode::Numpad1,
    VirtualKeyCode::Numpad2,
    VirtualKeyCode::Numpad3,
    VirtualKeyCode::Numpad4,
    VirtualKeyCode::Numpad5,
    VirtualKeyCode::Numpad6,
    VirtualKeyCode::Numpad7,
    VirtualKeyCode::Numpad8,
    VirtualKeyCode::Numpad9,
    VirtualKeyCode::AbntC1,
    VirtualKeyCode::AbntC2,
    VirtualKeyCode::Apostrophe,
    VirtualKeyCode::Apps,
    VirtualKeyCode::At,
    VirtualKeyCode::Ax,
    VirtualKeyCode::Backslash,
    VirtualKeyCode::Calculator,
    VirtualKeyCode::Capital,
    VirtualKeyCode::Colon,
    VirtualKeyCode::Comma,
    VirtualKeyCode::Convert,
    VirtualKeyCode::Equals,
    VirtualKeyCode::Grave,
    VirtualKeyCode::Kana,
    VirtualKeyCode::Kanji,
    VirtualKeyCode::LAlt,
    VirtualKeyCode::LBracket,
    VirtualKeyCode::LControl,
    VirtualKeyCode::LShift,
    VirtualKeyCode::LWin,
    VirtualKeyCode::Mail,
    VirtualKeyCode::MediaSelect,
    VirtualKeyCode::MediaStop,
    VirtualKeyCode::Minus,
    VirtualKeyCode::Mute,
    VirtualKeyCode::MyComputer,
    VirtualKeyCode::NavigateForward,
    VirtualKeyCode::NavigateBackward,
    VirtualKeyCode::NextTrack,
    VirtualKeyCode::NoConvert,
    VirtualKeyCode::NumpadComma,
    VirtualKeyCode::NumpadEnter,
    VirtualKeyCode::NumpadEquals,
    VirtualKeyCode::OEM102,
    VirtualKeyCode::Period,
    VirtualKeyCode::PlayPause,
    VirtualKeyCode::Power,
    VirtualKeyCode::PrevTrack,
    VirtualKeyCode::RAlt,
    VirtualKeyCode::RBracket,
    VirtualKeyCode::RControl,
    VirtualKeyCode::RShift,
    VirtualKeyCode::RWin,
    VirtualKeyCode::Semicolon,
    VirtualKeyCode::Slash,
    VirtualKeyCode::Sleep,
    VirtualKeyCode::Stop,
    VirtualKeyCode::Sysrq,
    VirtualKeyCode::Tab,
    VirtualKeyCode::Underline,
    VirtualKeyCode::Unlabeled,
    VirtualKeyCode::VolumeDown,
    VirtualKeyCode::VolumeUp,
    VirtualKeyCode::Wake,
];

fn create_keyboard_hash_map() -> HashMap<VirtualKeyCode, bool>{
    // constructs a hashmap with each key press as an index
    let mut keys: HashMap<VirtualKeyCode, bool> = HashMap::new();
    for k in &VIRTUALKEYS{
        keys.insert(*k, false);
    }
    keys
}

pub struct InputHandler{
    key_presses: HashMap<VirtualKeyCode, bool>, 
    mouse_position: [f32; 2],
    mouse_buttons: [bool; 3]
}

impl InputHandler{
    pub fn new_default() -> Self{
        Self{
            key_presses: create_keyboard_hash_map(),
            mouse_position: [0.0, 0.0],
            mouse_buttons: [false, false, false]
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool{
        match event{
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = [position.x as f32, position.y as f32];
                true
            },
            WindowEvent::KeyboardInput { input: KeyboardInput{virtual_keycode, state, ..}, .. } => {
                // state says whether the key was pressed or released
                // by using *state==winit::event::ElementState::Pressed,
                // the value will be set to false once it is released 
                // and will stay true as long as it is held
                self.key_presses.insert(virtual_keycode.unwrap(), *state==winit::event::ElementState::Pressed);
                true
            }
            _ => false
        }
    }

    pub fn get_mouse_pos(&self) -> [f32; 2]{
        self.mouse_position
    }

    // Get whether a specific key was pressed or not
    pub fn get_key_press(&self, key: &VirtualKeyCode) -> bool{
        // Try to return the value from the hashmap
        // if it's not in the hashmap (it will be), return false
        *self.key_presses.get(key).unwrap_or(&false)
    }

    // Returns a list of all pressed keys
    // Not fast, shouldn't be called a lot
    pub fn get_keys_pressed(&self) -> Vec<VirtualKeyCode>{
        self.key_presses
            .iter()
            // Remove any keys which aren't pressed
            .filter(|(_, press_state)| **press_state)
            // Drop the press state value
            .map(|(key_code, _)| *key_code)
            // turn into a Vec
            .collect()
    }
}