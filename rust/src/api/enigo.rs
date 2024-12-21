pub use enigo::Settings;
use enigo::{Key, Keyboard, Mouse};
use flutter_rust_bridge::*;
use std::sync::Mutex;

pub use enigo::{Axis, Coordinate, Direction};

#[frb(mirror(Settings))]
pub struct _Settings {
    pub linux_delay: u32,
    pub x11_display: Option<String>,
    pub wayland_display: Option<String>,
    pub windows_dw_extra_info: Option<usize>,
    pub event_source_user_data: Option<i64>,
    pub release_keys_when_dropped: bool,
    pub open_prompt_to_get_permissions: bool,
    pub independent_of_keyboard_state: bool,
    pub windows_subject_to_mouse_speed_and_acceleration_level: bool,
}

#[frb]
pub struct Enigo {
    #[frb(ignore)]
    enigo: Mutex<enigo::Enigo>,
}

unsafe impl Send for Enigo {}

unsafe impl Sync for Enigo {}

impl Enigo {
    #[frb(sync)]
    pub fn new(settings: &Settings) -> Self {
        Self {
            enigo: Mutex::new(enigo::Enigo::new(settings).unwrap()),
        }
    }

    #[frb(sync)]
    pub fn preset() -> Self {
        Self {
            enigo: Mutex::new(enigo::Enigo::new(&Settings::default()).unwrap()),
        }
    }

    #[frb(sync)]
    pub fn button(&mut self, button: _Button, direction: Direction) {
        self.enigo
            .lock()
            .unwrap()
            .button(button.value, direction)
            .unwrap();
    }

    #[frb(sync)]
    pub fn move_mouse(&mut self, x: i32, y: i32, coordinate: Coordinate) {
        self.enigo
            .lock()
            .unwrap()
            .move_mouse(x, y, coordinate)
            .unwrap();
    }

    #[frb(sync)]
    pub fn scroll(&mut self, length: i32, axis: Axis) {
        self.enigo.lock().unwrap().scroll(length, axis).unwrap();
    }

    #[frb(sync)]
    pub fn main_display(&self) -> (i32, i32) {
        self.enigo.lock().unwrap().main_display().unwrap()
    }

    #[frb(sync)]
    pub fn location(&self) -> (i32, i32) {
        self.enigo.lock().unwrap().location().unwrap()
    }

    #[frb(sync)]
    pub fn text(&mut self, text: &str) {
        self.enigo.lock().unwrap().text(text).unwrap();
    }

    #[frb(sync)]
    pub fn key(&mut self, key: Key, direction: Direction) {
        self.enigo.lock().unwrap().key(key, direction).unwrap();
    }

    #[frb(sync)]
    pub fn raw(&mut self, keycode: u16, direction: Direction) {
        self.enigo.lock().unwrap().raw(keycode, direction).unwrap()
    }
}

#[frb(mirror(Direction))]
pub enum _Direction {
    Press,
    Release,
    Click,
}

#[frb(mirror(Coordinate))]
pub enum _Coordinate {
    Abs,
    Rel,
}

#[frb(mirror(Axis))]
pub enum _Axis {
    Horizontal,
    Vertical,
}

#[frb(name = "Button")]
pub struct _Button {
    #[frb(ignore)]
    pub(crate) value: enigo::Button,
}

impl _Button {
    #[frb(sync)]
    pub fn new(value: &str) -> Self {
        Self {
            value: match value {
                "left" => enigo::Button::Left,
                "middle" => enigo::Button::Middle,
                "right" => enigo::Button::Right,
                #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
                "back" => enigo::Button::Back,
                #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
                "forward" => enigo::Button::Forward,
                "scroll_up" => enigo::Button::ScrollUp,
                "scroll_down" => enigo::Button::ScrollDown,
                "scroll_left" => enigo::Button::ScrollLeft,
                "scroll_right" => enigo::Button::ScrollRight,
                _ => panic!("Unspport!"),
            },
        }
    }

    #[frb(sync, getter)]
    pub fn left() -> Self {
        Self::new("left")
    }
    #[frb(sync, getter)]
    pub fn middle() -> Self {
        Self::new("middle")
    }
    #[frb(sync, getter)]
    pub fn right() -> Self {
        Self::new("right")
    }
    #[frb(sync, getter)]
    pub fn back() -> Self {
        Self::new("back")
    }
    #[frb(sync, getter)]
    pub fn forward() -> Self {
        Self::new("forward")
    }
    #[frb(sync, getter)]
    pub fn scroll_up() -> Self {
        Self::new("scroll_up")
    }

    #[frb(sync, getter)]
    pub fn scroll_down() -> Self {
        Self::new("scroll_down")
    }
    #[frb(sync, getter)]
    pub fn scroll_left() -> Self {
        Self::new("scroll_left")
    }
    #[frb(sync, getter)]
    pub fn scroll_right() -> Self {
        Self::new("scroll_right")
    }

    #[frb(sync)]
    pub fn to_string(&mut self) -> String {
        match self.value {
            enigo::Button::Left => "left",
            enigo::Button::Middle => "middle",
            enigo::Button::Right => "right",
            #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
            enigo::Button::Back => "back",
            #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
            enigo::Button::Forward => "forward",
            enigo::Button::ScrollUp => "scroll_up",
            enigo::Button::ScrollDown => "scroll_down",
            enigo::Button::ScrollLeft => "scroll_left",
            enigo::Button::ScrollRight => "scroll_right",
        }
        .to_string()
    }
}

#[frb(rust2dart(
    dart_type = "PhysicalKeyboardKey",
    dart_code = "PhysicalKeyboardKey({})"
))]
pub fn encode_physical_keyboard_key_type(raw: Key) -> u32 {
    match raw {
        Key::VolumeUp => 0x00070080,
        Key::VolumeDown => 0x00070081,
        Key::VolumeMute => 0x0007007f,
        #[cfg(target_os = "macos")]
        Key::BrightnessUp => 0x000c006f,
        #[cfg(target_os = "macos")]
        Key::BrightnessDown => 0x000c0070,
        #[cfg(target_os = "macos")]
        Key::Power => 0x00070066,
        #[cfg(target_os = "macos")]
        Key::LaunchPanel => 0x000c019f,
        #[cfg(target_os = "macos")]
        Key::Eject => 0x000c00b8,
        Key::MediaPlayPause => 0x000c00cd,
        Key::MediaNextTrack => 0x000c00b5,
        Key::MediaPrevTrack => 0x000c00b6,
        #[cfg(target_os = "macos")]
        Key::MediaFast => 0x000c0083,
        #[cfg(target_os = "macos")]
        Key::MediaRewind => 0x000c00b4,
        #[cfg(target_os = "macos")]
        Key::IlluminationUp => 0x000c0079,
        #[cfg(target_os = "macos")]
        Key::IlluminationDown => 0x000c007a,
        #[cfg(target_os = "macos")]
        Key::IlluminationToggle => 0x000c0072,
        Key::Other(value) => value,
        _ => 0,
    }
}

#[frb(dart2rust(dart_type = "PhysicalKeyboardKey", dart_code = "{}.usbHidUsage"))]
pub fn decode_physical_keyboard_key_type(raw: u32) -> Key {
    match raw {
        0x00070080 => Key::VolumeUp,
        0x00070081 => Key::VolumeDown,
        0x0007007f => Key::VolumeMute,
        #[cfg(target_os = "macos")]
        0x000c006f => Key::BrightnessUp,
        #[cfg(target_os = "macos")]
        0x000c0070 => Key::BrightnessDown,
        #[cfg(target_os = "macos")]
        0x00070066 => Key::Power,
        // => Key::ContrastUp ,
        // => Key::ContrastDown ,
        #[cfg(target_os = "macos")]
        0x000c019f => Key::LaunchPanel,
        #[cfg(target_os = "macos")]
        0x000c00b8 => Key::Eject,
        // => Key::VidMirror ,
        0x000c00cd => Key::MediaPlayPause,
        0x000c00b5 => Key::MediaNextTrack,
        0x000c00b6 => Key::MediaPrevTrack,
        #[cfg(target_os = "macos")]
        0x000c0083 => Key::MediaFast,
        #[cfg(target_os = "macos")]
        0x000c00b4 => Key::MediaRewind,
        #[cfg(target_os = "macos")]
        0x000c0079 => Key::IlluminationUp,
        #[cfg(target_os = "macos")]
        0x000c007a => Key::IlluminationDown,
        // TODO: 并未找到 IlluminationToggle 对应的 PhysicalKeyboardKey 值
        // 可能是 PhysicalKeyboardKey.brightnessToggle 吗？
        #[cfg(target_os = "macos")]
        0x000c0072 => Key::IlluminationToggle,
        _ => Key::Other(raw),
    }
}

#[frb(init)]
pub fn init_app() {
    setup_default_user_utils();
}
