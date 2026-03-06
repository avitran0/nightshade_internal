use egui::{Event, Key, Modifiers, PointerButton, Pos2, vec2};
use libc::c_char;

pub const SDL_KEYDOWN: u32 = 0x300;
pub const SDL_KEYUP: u32 = 0x301;
pub const SDL_TEXTINPUT: u32 = 0x303;
pub const SDL_MOUSEMOTION: u32 = 0x400;
pub const SDL_MOUSEBUTTONDOWN: u32 = 0x401;
pub const SDL_MOUSEBUTTONUP: u32 = 0x402;
pub const SDL_MOUSEWHEEL: u32 = 0x403;

#[repr(C)]
#[derive(Clone, Copy)]
pub union SdlEvent {
    pub kind: u32,
    pub mouse_motion: SdlMouseMotionEvent,
    pub mouse_button: SdlMouseButtonEvent,
    pub mouse_wheel: SdlMouseWheelEvent,
    pub keyboard: SdlKeyboardEvent,
    pub text_input: SdlTextInputEvent,
    pub padding: [u8; 56],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SdlMouseMotionEvent {
    pub kind: u32,
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub state: u32,
    pub x: i32,
    pub y: i32,
    pub x_rel: i32,
    pub y_rel: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SdlMouseButtonEvent {
    pub kind: u32,
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub button: u8,
    pub state: u8,
    pub clicks: u8,
    pub padding1: u8,
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SdlMouseWheelEvent {
    pub kind: u32,
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub x: i32,
    pub y: i32,
    pub direction: u32,
    pub precise_x: f32,
    pub precise_y: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SdlKeyboardEvent {
    pub kind: u32,
    pub timestamp: u32,
    pub window_id: u32,
    pub state: u8,
    pub repeat: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub keysym: SdlKeysym,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SdlKeysym {
    pub scancode: i32,
    pub sym: i32,
    pub mod_: u16,
    pub unused: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SdlTextInputEvent {
    pub kind: u32,
    pub timestamp: u32,
    pub window_id: u32,
    pub text: [c_char; 32],
}

impl SdlEvent {
    pub fn egui(&self) -> Option<egui::Event> {
        unsafe {
            match self.kind {
                SDL_MOUSEMOTION => {
                    let ev = self.mouse_motion;
                    Some(Event::PointerMoved(Pos2::new(ev.x as f32, ev.y as f32)))
                }
                SDL_MOUSEBUTTONDOWN | SDL_MOUSEBUTTONUP => {
                    let ev = self.mouse_button;
                    let button = match ev.button {
                        1 => Some(PointerButton::Primary),
                        2 => Some(PointerButton::Middle),
                        3 => Some(PointerButton::Secondary),
                        _ => None,
                    };
                    button.map(|b| Event::PointerButton {
                        pos: Pos2::new(ev.x as f32, ev.y as f32),
                        button: b,
                        pressed: ev.state == 1,
                        modifiers: Modifiers::default(),
                    })
                }
                SDL_MOUSEWHEEL => {
                    let ev = self.mouse_wheel;
                    let mut x = ev.x as f32 * 50.0;
                    let mut y = ev.y as f32 * 50.0;
                    if ev.direction == 1 {
                        x *= -1.0;
                        y *= -1.0;
                    }
                    Some(Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: vec2(x, y),
                        modifiers: Modifiers::default(),
                    })
                }
                SDL_KEYDOWN | SDL_KEYUP => {
                    let ev = self.keyboard;
                    let pressed = ev.state == 1;
                    keycode_to_egui(ev.keysym.sym).map(|key| Event::Key {
                        key,
                        physical_key: None,
                        pressed,
                        repeat: ev.repeat != 0,
                        modifiers: sdl_modifiers_to_egui(ev.keysym.mod_),
                    })
                }
                SDL_TEXTINPUT => {
                    let ev = self.text_input;
                    let c_str = std::ffi::CStr::from_ptr(ev.text.as_ptr());
                    if let Ok(s) = c_str.to_str() {
                        Some(Event::Text(s.to_string()))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }
}

fn sdl_modifiers_to_egui(mod_: u16) -> Modifiers {
    Modifiers {
        alt: (mod_ & 0x0100) != 0 || (mod_ & 0x0200) != 0,
        ctrl: (mod_ & 0x0040) != 0 || (mod_ & 0x0080) != 0,
        shift: (mod_ & 0x0001) != 0 || (mod_ & 0x0002) != 0,
        mac_cmd: false,
        command: (mod_ & 0x0040) != 0 || (mod_ & 0x0080) != 0,
    }
}

fn keycode_to_egui(sym: i32) -> Option<Key> {
    match sym {
        13 => Some(Key::Enter),
        27 => Some(Key::Escape),
        8 => Some(Key::Backspace),
        9 => Some(Key::Tab),
        32 => Some(Key::Space),
        127 => Some(Key::Delete),
        97..=122 => {
            // a-z -> A-Z
            let char_val = (sym as u8 as char).to_ascii_uppercase();
            Key::from_name(char_val.to_string().as_str())
        }
        48..=57 => {
            // 0-9
            let char_val = sym as u8 as char;
            Key::from_name(char_val.to_string().as_str())
        }
        _ => None,
    }
}
