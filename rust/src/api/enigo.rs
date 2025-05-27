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

#[cfg(target_os = "macos")]
#[frb(ignore)]
mod permission {
    use core_foundation::{
        base::TCFType,
        dictionary::{CFDictionary, CFDictionaryRef},
        string::{CFString, CFStringRef},
    };

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        pub fn AXIsProcessTrustedWithOptions(options: CFDictionaryRef) -> bool;
        static kAXTrustedCheckOptionPrompt: CFStringRef;
    }

    pub fn has_permission(open_prompt_to_get_permissions: bool) -> bool {
        let key = unsafe { kAXTrustedCheckOptionPrompt };
        let key = unsafe { CFString::wrap_under_create_rule(key) };

        let value = if open_prompt_to_get_permissions {
            core_foundation::boolean::CFBoolean::true_value()
        } else {
            core_foundation::boolean::CFBoolean::false_value()
        };

        let options = CFDictionary::from_CFType_pairs(&[(key, value)]);
        let options = options.as_concrete_TypeRef();
        unsafe { AXIsProcessTrustedWithOptions(options) }
    }
}
#[cfg(target_os = "windows")]
#[frb(ignore)]
mod permission {
    pub fn has_permission(open_prompt_to_get_permissions: bool) -> bool {
        true
    }
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
    pub fn has_permission(open_prompt: bool) -> bool {
        permission::has_permission(open_prompt)
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

// key code corresponding table
// https://github.com/flutter/flutter/blob/master/dev/tools/gen_keycodes/data/physical_key_data.g.json

#[frb(rust2dart(
    dart_type = "PhysicalKeyboardKey",
    dart_code = "PhysicalKeyboardKey({})"
))]
pub fn encode_physical_keyboard_key_type(raw: Key) -> u32 {
    match raw {
        // 需要特殊处理
        Key::VolumeUp => 0x00070080,
        Key::VolumeDown => 0x00070081,
        Key::VolumeMute => 0x0007007f,
        #[cfg(target_os = "macos")]
        Key::BrightnessUp => 0x000c0079,
        #[cfg(target_os = "macos")]
        Key::BrightnessDown => 0x000c007a,
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
        Key::IlluminationUp => 0x000c006f,
        #[cfg(target_os = "macos")]
        Key::IlluminationDown => 0x000c0070,
        #[cfg(target_os = "macos")]
        Key::IlluminationToggle => 0x000c0072,
        Key::Other(value) => match value {
            // Fn
            #[cfg(target_os = "macos")]
            63 => 0x00000012,
            // Sleep
            #[cfg(target_os = "windows")]
            57439 => 0x00010082,
            // WakeUp
            #[cfg(target_os = "windows")]
            57443 => 0x00010083,
            // UsbErrorRollOver
            #[cfg(target_os = "windows")]
            255 => 0x00070001,
            // UsbPostFail
            #[cfg(target_os = "windows")]
            252 => 0x00070002,
            // KeyA
            #[cfg(target_os = "macos")]
            0 => 0x00070004,
            #[cfg(target_os = "windows")]
            30 => 0x00070004,
            // KeyB
            #[cfg(target_os = "macos")]
            11 => 0x00070005,
            #[cfg(target_os = "windows")]
            48 => 0x00070005,
            // KeyC
            #[cfg(target_os = "macos")]
            8 => 0x00070006,
            #[cfg(target_os = "windows")]
            46 => 0x00070006,
            // KeyD
            #[cfg(target_os = "macos")]
            2 => 0x00070007,
            #[cfg(target_os = "windows")]
            32 => 0x00070007,
            // KeyE
            #[cfg(target_os = "macos")]
            14 => 0x00070008,
            #[cfg(target_os = "windows")]
            18 => 0x00070008,
            // KeyF
            #[cfg(target_os = "macos")]
            3 => 0x00070009,
            #[cfg(target_os = "windows")]
            33 => 0x00070009,
            // KeyG
            #[cfg(target_os = "macos")]
            5 => 0x0007000a,
            #[cfg(target_os = "windows")]
            34 => 0x0007000a,
            // KeyH
            #[cfg(target_os = "macos")]
            4 => 0x0007000b,
            #[cfg(target_os = "windows")]
            35 => 0x0007000b,
            // KeyI
            #[cfg(target_os = "macos")]
            34 => 0x0007000c,
            #[cfg(target_os = "windows")]
            23 => 0x0007000c,
            // KeyJ
            #[cfg(target_os = "macos")]
            38 => 0x0007000d,
            #[cfg(target_os = "windows")]
            36 => 0x0007000d,
            // KeyK
            #[cfg(target_os = "macos")]
            40 => 0x0007000e,
            #[cfg(target_os = "windows")]
            37 => 0x0007000e,
            // KeyL
            #[cfg(target_os = "macos")]
            37 => 0x0007000f,
            #[cfg(target_os = "windows")]
            38 => 0x0007000f,
            // KeyM
            #[cfg(target_os = "macos")]
            46 => 0x00070010,
            #[cfg(target_os = "windows")]
            50 => 0x00070010,
            // KeyN
            #[cfg(target_os = "macos")]
            45 => 0x00070011,
            #[cfg(target_os = "windows")]
            49 => 0x00070011,
            // KeyO
            #[cfg(target_os = "macos")]
            31 => 0x00070012,
            #[cfg(target_os = "windows")]
            24 => 0x00070012,
            // KeyP
            #[cfg(target_os = "macos")]
            35 => 0x00070013,
            #[cfg(target_os = "windows")]
            25 => 0x00070013,
            // KeyQ
            #[cfg(target_os = "macos")]
            12 => 0x00070014,
            #[cfg(target_os = "windows")]
            16 => 0x00070014,
            // KeyR
            #[cfg(target_os = "macos")]
            15 => 0x00070015,
            #[cfg(target_os = "windows")]
            19 => 0x00070015,
            // KeyS
            #[cfg(target_os = "macos")]
            1 => 0x00070016,
            #[cfg(target_os = "windows")]
            31 => 0x00070016,
            // KeyT
            #[cfg(target_os = "macos")]
            17 => 0x00070017,
            #[cfg(target_os = "windows")]
            20 => 0x00070017,
            // KeyU
            #[cfg(target_os = "macos")]
            32 => 0x00070018,
            #[cfg(target_os = "windows")]
            22 => 0x00070018,
            // KeyV
            #[cfg(target_os = "macos")]
            9 => 0x00070019,
            #[cfg(target_os = "windows")]
            47 => 0x00070019,
            // KeyW
            #[cfg(target_os = "macos")]
            13 => 0x0007001a,
            #[cfg(target_os = "windows")]
            17 => 0x0007001a,
            // KeyX
            #[cfg(target_os = "macos")]
            7 => 0x0007001b,
            #[cfg(target_os = "windows")]
            45 => 0x0007001b,
            // KeyY
            #[cfg(target_os = "macos")]
            16 => 0x0007001c,
            #[cfg(target_os = "windows")]
            21 => 0x0007001c,
            // KeyZ
            #[cfg(target_os = "macos")]
            6 => 0x0007001d,
            #[cfg(target_os = "windows")]
            44 => 0x0007001d,
            // Digit1
            #[cfg(target_os = "macos")]
            18 => 0x0007001e,
            #[cfg(target_os = "windows")]
            2 => 0x0007001e,
            // Digit2
            #[cfg(target_os = "macos")]
            19 => 0x0007001f,
            #[cfg(target_os = "windows")]
            3 => 0x0007001f,
            // Digit3
            #[cfg(target_os = "macos")]
            20 => 0x00070020,
            #[cfg(target_os = "windows")]
            4 => 0x00070020,
            // Digit4
            #[cfg(target_os = "macos")]
            21 => 0x00070021,
            #[cfg(target_os = "windows")]
            5 => 0x00070021,
            // Digit5
            #[cfg(target_os = "macos")]
            23 => 0x00070022,
            #[cfg(target_os = "windows")]
            6 => 0x00070022,
            // Digit6
            #[cfg(target_os = "macos")]
            22 => 0x00070023,
            #[cfg(target_os = "windows")]
            7 => 0x00070023,
            // Digit7
            #[cfg(target_os = "macos")]
            26 => 0x00070024,
            #[cfg(target_os = "windows")]
            8 => 0x00070024,
            // Digit8
            #[cfg(target_os = "macos")]
            28 => 0x00070025,
            #[cfg(target_os = "windows")]
            9 => 0x00070025,
            // Digit9
            #[cfg(target_os = "macos")]
            25 => 0x00070026,
            #[cfg(target_os = "windows")]
            10 => 0x00070026,
            // Digit0
            #[cfg(target_os = "macos")]
            29 => 0x00070027,
            #[cfg(target_os = "windows")]
            11 => 0x00070027,
            // Enter
            #[cfg(target_os = "macos")]
            36 => 0x00070028,
            #[cfg(target_os = "windows")]
            28 => 0x00070028,
            // Escape
            #[cfg(target_os = "macos")]
            53 => 0x00070029,
            #[cfg(target_os = "windows")]
            1 => 0x00070029,
            // Backspace
            #[cfg(target_os = "macos")]
            51 => 0x0007002a,
            #[cfg(target_os = "windows")]
            14 => 0x0007002a,
            // Tab
            #[cfg(target_os = "macos")]
            48 => 0x0007002b,
            #[cfg(target_os = "windows")]
            15 => 0x0007002b,
            // Space
            #[cfg(target_os = "macos")]
            49 => 0x0007002c,
            #[cfg(target_os = "windows")]
            57 => 0x0007002c,
            // Minus
            #[cfg(target_os = "macos")]
            27 => 0x0007002d,
            #[cfg(target_os = "windows")]
            12 => 0x0007002d,
            // Equal
            #[cfg(target_os = "macos")]
            24 => 0x0007002e,
            #[cfg(target_os = "windows")]
            13 => 0x0007002e,
            // BracketLeft
            #[cfg(target_os = "macos")]
            33 => 0x0007002f,
            #[cfg(target_os = "windows")]
            26 => 0x0007002f,
            // BracketRight
            #[cfg(target_os = "macos")]
            30 => 0x00070030,
            #[cfg(target_os = "windows")]
            27 => 0x00070030,
            // Backslash
            #[cfg(target_os = "macos")]
            42 => 0x00070031,
            #[cfg(target_os = "windows")]
            43 => 0x00070031,
            // Semicolon
            #[cfg(target_os = "macos")]
            41 => 0x00070033,
            #[cfg(target_os = "windows")]
            39 => 0x00070033,
            // Quote
            #[cfg(target_os = "macos")]
            39 => 0x00070034,
            #[cfg(target_os = "windows")]
            40 => 0x00070034,
            // Backquote
            #[cfg(target_os = "macos")]
            50 => 0x00070035,
            #[cfg(target_os = "windows")]
            41 => 0x00070035,
            // Comma
            #[cfg(target_os = "macos")]
            43 => 0x00070036,
            #[cfg(target_os = "windows")]
            51 => 0x00070036,
            // Period
            #[cfg(target_os = "macos")]
            47 => 0x00070037,
            #[cfg(target_os = "windows")]
            52 => 0x00070037,
            // Slash
            #[cfg(target_os = "macos")]
            44 => 0x00070038,
            #[cfg(target_os = "windows")]
            53 => 0x00070038,
            // CapsLock
            #[cfg(target_os = "macos")]
            57 => 0x00070039,
            #[cfg(target_os = "windows")]
            58 => 0x00070039,
            // F1
            #[cfg(target_os = "macos")]
            122 => 0x0007003a,
            #[cfg(target_os = "windows")]
            59 => 0x0007003a,
            // F2
            #[cfg(target_os = "macos")]
            120 => 0x0007003b,
            #[cfg(target_os = "windows")]
            60 => 0x0007003b,
            // F3
            #[cfg(target_os = "macos")]
            99 => 0x0007003c,
            #[cfg(target_os = "windows")]
            61 => 0x0007003c,
            // F4
            #[cfg(target_os = "macos")]
            118 => 0x0007003d,
            #[cfg(target_os = "windows")]
            62 => 0x0007003d,
            // F5
            #[cfg(target_os = "macos")]
            96 => 0x0007003e,
            #[cfg(target_os = "windows")]
            63 => 0x0007003e,
            // F6
            #[cfg(target_os = "macos")]
            97 => 0x0007003f,
            #[cfg(target_os = "windows")]
            64 => 0x0007003f,
            // F7
            #[cfg(target_os = "macos")]
            98 => 0x00070040,
            #[cfg(target_os = "windows")]
            65 => 0x00070040,
            // F8
            #[cfg(target_os = "macos")]
            100 => 0x00070041,
            #[cfg(target_os = "windows")]
            66 => 0x00070041,
            // F9
            #[cfg(target_os = "macos")]
            101 => 0x00070042,
            #[cfg(target_os = "windows")]
            67 => 0x00070042,
            // F10
            #[cfg(target_os = "macos")]
            109 => 0x00070043,
            #[cfg(target_os = "windows")]
            68 => 0x00070043,
            // F11
            #[cfg(target_os = "macos")]
            103 => 0x00070044,
            #[cfg(target_os = "windows")]
            87 => 0x00070044,
            // F12
            #[cfg(target_os = "macos")]
            111 => 0x00070045,
            #[cfg(target_os = "windows")]
            88 => 0x00070045,
            // PrintScreen
            #[cfg(target_os = "windows")]
            57399 => 0x00070046,
            // ScrollLock
            #[cfg(target_os = "windows")]
            70 => 0x00070047,
            // Pause
            #[cfg(target_os = "windows")]
            69 => 0x00070048,
            // Insert
            #[cfg(target_os = "macos")]
            114 => 0x00070049,
            #[cfg(target_os = "windows")]
            57426 => 0x00070049,
            // Home
            #[cfg(target_os = "macos")]
            115 => 0x0007004a,
            #[cfg(target_os = "windows")]
            57415 => 0x0007004a,
            // PageUp
            #[cfg(target_os = "macos")]
            116 => 0x0007004b,
            #[cfg(target_os = "windows")]
            57417 => 0x0007004b,
            // Delete
            #[cfg(target_os = "macos")]
            117 => 0x0007004c,
            #[cfg(target_os = "windows")]
            57427 => 0x0007004c,
            // End
            #[cfg(target_os = "macos")]
            119 => 0x0007004d,
            #[cfg(target_os = "windows")]
            57423 => 0x0007004d,
            // PageDown
            #[cfg(target_os = "macos")]
            121 => 0x0007004e,
            #[cfg(target_os = "windows")]
            57425 => 0x0007004e,
            // ArrowRight
            #[cfg(target_os = "macos")]
            124 => 0x0007004f,
            #[cfg(target_os = "windows")]
            57421 => 0x0007004f,
            // ArrowLeft
            #[cfg(target_os = "macos")]
            123 => 0x00070050,
            #[cfg(target_os = "windows")]
            57419 => 0x00070050,
            // ArrowDown
            #[cfg(target_os = "macos")]
            125 => 0x00070051,
            #[cfg(target_os = "windows")]
            57424 => 0x00070051,
            // ArrowUp
            #[cfg(target_os = "macos")]
            126 => 0x00070052,
            #[cfg(target_os = "windows")]
            57416 => 0x00070052,
            // NumLock
            #[cfg(target_os = "macos")]
            71 => 0x00070053,
            #[cfg(target_os = "windows")]
            57413 => 0x00070053,
            // NumpadDivide
            #[cfg(target_os = "macos")]
            75 => 0x00070054,
            #[cfg(target_os = "windows")]
            57397 => 0x00070054,
            // NumpadMultiply
            #[cfg(target_os = "macos")]
            67 => 0x00070055,
            #[cfg(target_os = "windows")]
            55 => 0x00070055,
            // NumpadSubtract
            #[cfg(target_os = "macos")]
            78 => 0x00070056,
            #[cfg(target_os = "windows")]
            74 => 0x00070056,
            // NumpadAdd
            #[cfg(target_os = "macos")]
            69 => 0x00070057,
            #[cfg(target_os = "windows")]
            78 => 0x00070057,
            // NumpadEnter
            #[cfg(target_os = "macos")]
            76 => 0x00070058,
            #[cfg(target_os = "windows")]
            57372 => 0x00070058,
            // Numpad1
            #[cfg(target_os = "macos")]
            83 => 0x00070059,
            #[cfg(target_os = "windows")]
            79 => 0x00070059,
            // Numpad2
            #[cfg(target_os = "macos")]
            84 => 0x0007005a,
            #[cfg(target_os = "windows")]
            80 => 0x0007005a,
            // Numpad3
            #[cfg(target_os = "macos")]
            85 => 0x0007005b,
            #[cfg(target_os = "windows")]
            81 => 0x0007005b,
            // Numpad4
            #[cfg(target_os = "macos")]
            86 => 0x0007005c,
            #[cfg(target_os = "windows")]
            75 => 0x0007005c,
            // Numpad5
            #[cfg(target_os = "macos")]
            87 => 0x0007005d,
            #[cfg(target_os = "windows")]
            76 => 0x0007005d,
            // Numpad6
            #[cfg(target_os = "macos")]
            88 => 0x0007005e,
            #[cfg(target_os = "windows")]
            77 => 0x0007005e,
            // Numpad7
            #[cfg(target_os = "macos")]
            89 => 0x0007005f,
            #[cfg(target_os = "windows")]
            71 => 0x0007005f,
            // Numpad8
            #[cfg(target_os = "macos")]
            91 => 0x00070060,
            #[cfg(target_os = "windows")]
            72 => 0x00070060,
            // Numpad9
            #[cfg(target_os = "macos")]
            92 => 0x00070061,
            #[cfg(target_os = "windows")]
            73 => 0x00070061,
            // Numpad0
            82 => 0x00070062,
            // NumpadDecimal
            #[cfg(target_os = "macos")]
            65 => 0x00070063,
            #[cfg(target_os = "windows")]
            83 => 0x00070063,
            // IntlBackslash
            #[cfg(target_os = "macos")]
            10 => 0x00070064,
            #[cfg(target_os = "windows")]
            86 => 0x00070064,
            // ContextMenu
            #[cfg(target_os = "macos")]
            110 => 0x00070065,
            #[cfg(target_os = "windows")]
            57437 => 0x00070065,
            // Power
            #[cfg(target_os = "windows")]
            57438 => 0x00070066,
            // NumpadEqual
            #[cfg(target_os = "macos")]
            81 => 0x00070067,
            #[cfg(target_os = "windows")]
            89 => 0x00070067,
            // F13
            #[cfg(target_os = "macos")]
            105 => 0x00070068,
            #[cfg(target_os = "windows")]
            100 => 0x00070068,
            // F14
            #[cfg(target_os = "macos")]
            107 => 0x00070069,
            #[cfg(target_os = "windows")]
            101 => 0x00070069,
            // F15
            #[cfg(target_os = "macos")]
            113 => 0x0007006a,
            #[cfg(target_os = "windows")]
            102 => 0x0007006a,
            // F16
            #[cfg(target_os = "macos")]
            106 => 0x0007006b,
            #[cfg(target_os = "windows")]
            103 => 0x0007006b,
            // F17
            #[cfg(target_os = "macos")]
            64 => 0x0007006c,
            #[cfg(target_os = "windows")]
            104 => 0x0007006c,
            // F18
            #[cfg(target_os = "macos")]
            79 => 0x0007006d,
            #[cfg(target_os = "windows")]
            105 => 0x0007006d,
            // F19
            #[cfg(target_os = "macos")]
            80 => 0x0007006e,
            #[cfg(target_os = "windows")]
            106 => 0x0007006e,
            // F20
            #[cfg(target_os = "macos")]
            90 => 0x0007006f,
            #[cfg(target_os = "windows")]
            107 => 0x0007006f,
            // F21
            #[cfg(target_os = "windows")]
            108 => 0x00070070,
            // F22
            #[cfg(target_os = "windows")]
            109 => 0x00070071,
            // F23
            #[cfg(target_os = "windows")]
            110 => 0x00070072,
            // F24
            #[cfg(target_os = "windows")]
            118 => 0x00070073,
            // Help
            #[cfg(target_os = "windows")]
            57403 => 0x00070075,
            // Undo
            #[cfg(target_os = "windows")]
            57352 => 0x0007007a,
            // Cut
            #[cfg(target_os = "windows")]
            57367 => 0x0007007b,
            // Copy
            #[cfg(target_os = "windows")]
            57368 => 0x0007007c,
            // Paste
            #[cfg(target_os = "windows")]
            57354 => 0x0007007d,
            // NumpadComma
            #[cfg(target_os = "macos")]
            95 => 0x00070085,
            #[cfg(target_os = "windows")]
            126 => 0x00070085,
            // IntlRo
            #[cfg(target_os = "macos")]
            94 => 0x00070087,
            #[cfg(target_os = "windows")]
            115 => 0x00070087,
            // KanaMode
            #[cfg(target_os = "windows")]
            112 => 0x00070088,
            // IntlYen
            #[cfg(target_os = "macos")]
            93 => 0x00070089,
            #[cfg(target_os = "windows")]
            125 => 0x00070089,
            // Convert
            #[cfg(target_os = "windows")]
            121 => 0x0007008a,
            // NonConvert
            #[cfg(target_os = "windows")]
            123 => 0x0007008b,
            // Lang1
            #[cfg(target_os = "macos")]
            104 => 0x00070090,
            #[cfg(target_os = "windows")]
            114 => 0x00070090,
            // Lang2
            #[cfg(target_os = "macos")]
            102 => 0x00070091,
            #[cfg(target_os = "windows")]
            113 => 0x00070091,
            // Lang3
            #[cfg(target_os = "windows")]
            120 => 0x00070092,
            // Lang4
            #[cfg(target_os = "windows")]
            119 => 0x00070093,
            // ControlLeft
            #[cfg(target_os = "macos")]
            59 => 0x000700e0,
            #[cfg(target_os = "windows")]
            29 => 0x000700e0,
            // ShiftLeft
            #[cfg(target_os = "macos")]
            56 => 0x000700e1,
            #[cfg(target_os = "windows")]
            42 => 0x000700e1,
            // AltLeft
            #[cfg(target_os = "macos")]
            58 => 0x000700e2,
            #[cfg(target_os = "windows")]
            56 => 0x000700e2,
            // MetaLeft
            #[cfg(target_os = "macos")]
            55 => 0x000700e3,
            #[cfg(target_os = "windows")]
            57435 => 0x000700e3,
            // ControlRight
            #[cfg(target_os = "macos")]
            62 => 0x000700e4,
            #[cfg(target_os = "windows")]
            57373 => 0x000700e4,
            // ShiftRight
            #[cfg(target_os = "macos")]
            60 => 0x000700e5,
            #[cfg(target_os = "windows")]
            54 => 0x000700e5,
            // AltRight
            #[cfg(target_os = "macos")]
            61 => 0x000700e6,
            #[cfg(target_os = "windows")]
            57400 => 0x000700e6,
            // MetaRight
            #[cfg(target_os = "macos")]
            54 => 0x000700e7,
            #[cfg(target_os = "windows")]
            57436 => 0x000700e7,
            // MediaStop
            #[cfg(target_os = "windows")]
            57380 => 0x000c00b7,
            // Eject
            #[cfg(target_os = "windows")]
            57388 => 0x000c00b8,
            // MediaSelect
            #[cfg(target_os = "windows")]
            57453 => 0x000c0183,
            // LaunchMail
            #[cfg(target_os = "windows")]
            57452 => 0x000c018a,
            // LaunchApp2
            #[cfg(target_os = "windows")]
            57377 => 0x000c0192,
            // LaunchApp1
            #[cfg(target_os = "windows")]
            57451 => 0x000c0194,
            // BrowserSearch
            #[cfg(target_os = "windows")]
            57445 => 0x000c0221,
            // BrowserHome
            #[cfg(target_os = "windows")]
            57394 => 0x000c0223,
            // BrowserBack
            #[cfg(target_os = "windows")]
            57450 => 0x000c0224,
            // BrowserForward
            #[cfg(target_os = "windows")]
            57449 => 0x000c0225,
            // BrowserStop
            #[cfg(target_os = "windows")]
            57448 => 0x000c0226,
            // BrowserRefresh
            #[cfg(target_os = "windows")]
            57447 => 0x000c0227,
            // BrowserFavorites
            #[cfg(target_os = "windows")]
            57446 => 0x000c022a,
            _ => 0,
        },
        _ => 0,
    }
}

#[frb(dart2rust(dart_type = "PhysicalKeyboardKey", dart_code = "{}.usbHidUsage"))]
pub fn decode_physical_keyboard_key_type(raw: u32) -> Key {
    match raw {
        // Fn
        #[cfg(target_os = "macos")]
        0x00000012 => Key::Other(63),
        // Sleep
        #[cfg(target_os = "windows")]
        0x00010082 => Key::Other(57439),
        // WakeUp
        #[cfg(target_os = "windows")]
        0x00010083 => Key::Other(57443),
        // UsbErrorRollOver
        #[cfg(target_os = "windows")]
        0x00070001 => Key::Other(255),
        // UsbPostFail
        #[cfg(target_os = "windows")]
        0x00070002 => Key::Other(252),
        // KeyA
        #[cfg(target_os = "macos")]
        0x00070004 => Key::Other(0),
        #[cfg(target_os = "windows")]
        0x00070004 => Key::Other(30),
        // KeyB
        #[cfg(target_os = "macos")]
        0x00070005 => Key::Other(11),
        #[cfg(target_os = "windows")]
        0x00070005 => Key::Other(48),
        // KeyC
        #[cfg(target_os = "macos")]
        0x00070006 => Key::Other(8),
        #[cfg(target_os = "windows")]
        0x00070006 => Key::Other(46),
        // KeyD
        #[cfg(target_os = "macos")]
        0x00070007 => Key::Other(2),
        #[cfg(target_os = "windows")]
        0x00070007 => Key::Other(32),
        // KeyE
        #[cfg(target_os = "macos")]
        0x00070008 => Key::Other(14),
        #[cfg(target_os = "windows")]
        0x00070008 => Key::Other(18),
        // KeyF
        #[cfg(target_os = "macos")]
        0x00070009 => Key::Other(3),
        #[cfg(target_os = "windows")]
        0x00070009 => Key::Other(33),
        // KeyG
        #[cfg(target_os = "macos")]
        0x0007000a => Key::Other(5),
        #[cfg(target_os = "windows")]
        0x0007000a => Key::Other(34),
        // KeyH
        #[cfg(target_os = "macos")]
        0x0007000b => Key::Other(4),
        #[cfg(target_os = "windows")]
        0x0007000b => Key::Other(35),
        // KeyI
        #[cfg(target_os = "macos")]
        0x0007000c => Key::Other(34),
        #[cfg(target_os = "windows")]
        0x0007000c => Key::Other(23),
        // KeyJ
        #[cfg(target_os = "macos")]
        0x0007000d => Key::Other(38),
        #[cfg(target_os = "windows")]
        0x0007000d => Key::Other(36),
        // KeyK
        #[cfg(target_os = "macos")]
        0x0007000e => Key::Other(40),
        #[cfg(target_os = "windows")]
        0x0007000e => Key::Other(37),
        // KeyL
        #[cfg(target_os = "macos")]
        0x0007000f => Key::Other(37),
        #[cfg(target_os = "windows")]
        0x0007000f => Key::Other(38),
        // KeyM
        #[cfg(target_os = "macos")]
        0x00070010 => Key::Other(46),
        #[cfg(target_os = "windows")]
        0x00070010 => Key::Other(50),
        // KeyN
        #[cfg(target_os = "macos")]
        0x00070011 => Key::Other(45),
        #[cfg(target_os = "windows")]
        0x00070011 => Key::Other(49),
        // KeyO
        #[cfg(target_os = "macos")]
        0x00070012 => Key::Other(31),
        #[cfg(target_os = "windows")]
        0x00070012 => Key::Other(24),
        // KeyP
        #[cfg(target_os = "macos")]
        0x00070013 => Key::Other(35),
        #[cfg(target_os = "windows")]
        0x00070013 => Key::Other(25),
        // KeyQ
        #[cfg(target_os = "macos")]
        0x00070014 => Key::Other(12),
        #[cfg(target_os = "windows")]
        0x00070014 => Key::Other(16),
        // KeyR
        #[cfg(target_os = "macos")]
        0x00070015 => Key::Other(15),
        #[cfg(target_os = "windows")]
        0x00070015 => Key::Other(19),
        // KeyS
        #[cfg(target_os = "macos")]
        0x00070016 => Key::Other(1),
        #[cfg(target_os = "windows")]
        0x00070016 => Key::Other(31),
        // KeyT
        #[cfg(target_os = "macos")]
        0x00070017 => Key::Other(17),
        #[cfg(target_os = "windows")]
        0x00070017 => Key::Other(20),
        // KeyU
        #[cfg(target_os = "macos")]
        0x00070018 => Key::Other(32),
        #[cfg(target_os = "windows")]
        0x00070018 => Key::Other(22),
        // KeyV
        #[cfg(target_os = "macos")]
        0x00070019 => Key::Other(9),
        #[cfg(target_os = "windows")]
        0x00070019 => Key::Other(47),
        // KeyW
        #[cfg(target_os = "macos")]
        0x0007001a => Key::Other(13),
        #[cfg(target_os = "windows")]
        0x0007001a => Key::Other(17),
        // KeyX
        #[cfg(target_os = "macos")]
        0x0007001b => Key::Other(7),
        #[cfg(target_os = "windows")]
        0x0007001b => Key::Other(45),
        // KeyY
        #[cfg(target_os = "macos")]
        0x0007001c => Key::Other(16),
        #[cfg(target_os = "windows")]
        0x0007001c => Key::Other(21),
        // KeyZ
        #[cfg(target_os = "macos")]
        0x0007001d => Key::Other(6),
        #[cfg(target_os = "windows")]
        0x0007001d => Key::Other(44),
        // Digit1
        #[cfg(target_os = "macos")]
        0x0007001e => Key::Other(18),
        #[cfg(target_os = "windows")]
        0x0007001e => Key::Other(2),
        // Digit2
        #[cfg(target_os = "macos")]
        0x0007001f => Key::Other(19),
        #[cfg(target_os = "windows")]
        0x0007001f => Key::Other(3),
        // Digit3
        #[cfg(target_os = "macos")]
        0x00070020 => Key::Other(20),
        #[cfg(target_os = "windows")]
        0x00070020 => Key::Other(4),
        // Digit4
        #[cfg(target_os = "macos")]
        0x00070021 => Key::Other(21),
        #[cfg(target_os = "windows")]
        0x00070021 => Key::Other(5),
        // Digit5
        #[cfg(target_os = "macos")]
        0x00070022 => Key::Other(23),
        #[cfg(target_os = "windows")]
        0x00070022 => Key::Other(6),
        // Digit6
        #[cfg(target_os = "macos")]
        0x00070023 => Key::Other(22),
        #[cfg(target_os = "windows")]
        0x00070023 => Key::Other(7),
        // Digit7
        #[cfg(target_os = "macos")]
        0x00070024 => Key::Other(26),
        #[cfg(target_os = "windows")]
        0x00070024 => Key::Other(8),
        // Digit8
        #[cfg(target_os = "macos")]
        0x00070025 => Key::Other(28),
        #[cfg(target_os = "windows")]
        0x00070025 => Key::Other(9),
        // Digit9
        #[cfg(target_os = "macos")]
        0x00070026 => Key::Other(25),
        #[cfg(target_os = "windows")]
        0x00070026 => Key::Other(10),
        // Digit0
        #[cfg(target_os = "macos")]
        0x00070027 => Key::Other(29),
        #[cfg(target_os = "windows")]
        0x00070027 => Key::Other(11),
        // Enter
        #[cfg(target_os = "macos")]
        0x00070028 => Key::Other(36),
        #[cfg(target_os = "windows")]
        0x00070028 => Key::Other(28),
        // Escape
        #[cfg(target_os = "macos")]
        0x00070029 => Key::Other(53),
        #[cfg(target_os = "windows")]
        0x00070029 => Key::Other(1),
        // Backspace
        #[cfg(target_os = "macos")]
        0x0007002a => Key::Other(51),
        #[cfg(target_os = "windows")]
        0x0007002a => Key::Other(14),
        // Tab
        #[cfg(target_os = "macos")]
        0x0007002b => Key::Other(48),
        #[cfg(target_os = "windows")]
        0x0007002b => Key::Other(15),
        // Space
        #[cfg(target_os = "macos")]
        0x0007002c => Key::Other(49),
        #[cfg(target_os = "windows")]
        0x0007002c => Key::Other(57),
        // Minus
        #[cfg(target_os = "macos")]
        0x0007002d => Key::Other(27),
        #[cfg(target_os = "windows")]
        0x0007002d => Key::Other(12),
        // Equal
        #[cfg(target_os = "macos")]
        0x0007002e => Key::Other(24),
        #[cfg(target_os = "windows")]
        0x0007002e => Key::Other(13),
        // BracketLeft
        #[cfg(target_os = "macos")]
        0x0007002f => Key::Other(33),
        #[cfg(target_os = "windows")]
        0x0007002f => Key::Other(26),
        // BracketRight
        #[cfg(target_os = "macos")]
        0x00070030 => Key::Other(30),
        #[cfg(target_os = "windows")]
        0x00070030 => Key::Other(27),
        // Backslash
        #[cfg(target_os = "macos")]
        0x00070031 => Key::Other(42),
        #[cfg(target_os = "windows")]
        0x00070031 => Key::Other(43),
        // Semicolon
        #[cfg(target_os = "macos")]
        0x00070033 => Key::Other(41),
        #[cfg(target_os = "windows")]
        0x00070033 => Key::Other(39),
        // Quote
        #[cfg(target_os = "macos")]
        0x00070034 => Key::Other(39),
        #[cfg(target_os = "windows")]
        0x00070034 => Key::Other(40),
        // Backquote
        #[cfg(target_os = "macos")]
        0x00070035 => Key::Other(50),
        #[cfg(target_os = "windows")]
        0x00070035 => Key::Other(41),
        // Comma
        #[cfg(target_os = "macos")]
        0x00070036 => Key::Other(43),
        #[cfg(target_os = "windows")]
        0x00070036 => Key::Other(51),
        // Period
        #[cfg(target_os = "macos")]
        0x00070037 => Key::Other(47),
        #[cfg(target_os = "windows")]
        0x00070037 => Key::Other(52),
        // Slash
        #[cfg(target_os = "macos")]
        0x00070038 => Key::Other(44),
        #[cfg(target_os = "windows")]
        0x00070038 => Key::Other(53),
        // CapsLock
        #[cfg(target_os = "macos")]
        0x00070039 => Key::Other(57),
        #[cfg(target_os = "windows")]
        0x00070039 => Key::Other(58),
        // F1
        #[cfg(target_os = "macos")]
        0x0007003a => Key::Other(122),
        #[cfg(target_os = "windows")]
        0x0007003a => Key::Other(59),
        // F2
        #[cfg(target_os = "macos")]
        0x0007003b => Key::Other(120),
        #[cfg(target_os = "windows")]
        0x0007003b => Key::Other(60),
        // F3
        #[cfg(target_os = "macos")]
        0x0007003c => Key::Other(99),
        #[cfg(target_os = "windows")]
        0x0007003c => Key::Other(61),
        // F4
        #[cfg(target_os = "macos")]
        0x0007003d => Key::Other(118),
        #[cfg(target_os = "windows")]
        0x0007003d => Key::Other(62),
        // F5
        #[cfg(target_os = "macos")]
        0x0007003e => Key::Other(96),
        #[cfg(target_os = "windows")]
        0x0007003e => Key::Other(63),
        // F6
        #[cfg(target_os = "macos")]
        0x0007003f => Key::Other(97),
        #[cfg(target_os = "windows")]
        0x0007003f => Key::Other(64),
        // F7
        #[cfg(target_os = "macos")]
        0x00070040 => Key::Other(98),
        #[cfg(target_os = "windows")]
        0x00070040 => Key::Other(65),
        // F8
        #[cfg(target_os = "macos")]
        0x00070041 => Key::Other(100),
        #[cfg(target_os = "windows")]
        0x00070041 => Key::Other(66),
        // F9
        #[cfg(target_os = "macos")]
        0x00070042 => Key::Other(101),
        #[cfg(target_os = "windows")]
        0x00070042 => Key::Other(67),
        // F10
        #[cfg(target_os = "macos")]
        0x00070043 => Key::Other(109),
        #[cfg(target_os = "windows")]
        0x00070043 => Key::Other(68),
        // F11
        #[cfg(target_os = "macos")]
        0x00070044 => Key::Other(103),
        #[cfg(target_os = "windows")]
        0x00070044 => Key::Other(87),
        // F12
        #[cfg(target_os = "macos")]
        0x00070045 => Key::Other(111),
        #[cfg(target_os = "windows")]
        0x00070045 => Key::Other(88),
        // PrintScreen
        #[cfg(target_os = "windows")]
        0x00070046 => Key::Other(57399),
        // ScrollLock
        #[cfg(target_os = "windows")]
        0x00070047 => Key::Other(70),
        // Pause
        #[cfg(target_os = "windows")]
        0x00070048 => Key::Other(69),
        // Insert
        #[cfg(target_os = "macos")]
        0x00070049 => Key::Other(114),
        #[cfg(target_os = "windows")]
        0x00070049 => Key::Other(57426),
        // Home
        #[cfg(target_os = "macos")]
        0x0007004a => Key::Other(115),
        #[cfg(target_os = "windows")]
        0x0007004a => Key::Other(57415),
        // PageUp
        #[cfg(target_os = "macos")]
        0x0007004b => Key::Other(116),
        #[cfg(target_os = "windows")]
        0x0007004b => Key::Other(57417),
        // Delete
        #[cfg(target_os = "macos")]
        0x0007004c => Key::Other(117),
        #[cfg(target_os = "windows")]
        0x0007004c => Key::Other(57427),
        // End
        #[cfg(target_os = "macos")]
        0x0007004d => Key::Other(119),
        #[cfg(target_os = "windows")]
        0x0007004d => Key::Other(57423),
        // PageDown
        #[cfg(target_os = "macos")]
        0x0007004e => Key::Other(121),
        #[cfg(target_os = "windows")]
        0x0007004e => Key::Other(57425),
        // ArrowRight
        #[cfg(target_os = "macos")]
        0x0007004f => Key::Other(124),
        #[cfg(target_os = "windows")]
        0x0007004f => Key::Other(57421),
        // ArrowLeft
        #[cfg(target_os = "macos")]
        0x00070050 => Key::Other(123),
        #[cfg(target_os = "windows")]
        0x00070050 => Key::Other(57419),
        // ArrowDown
        #[cfg(target_os = "macos")]
        0x00070051 => Key::Other(125),
        #[cfg(target_os = "windows")]
        0x00070051 => Key::Other(57424),
        // ArrowUp
        #[cfg(target_os = "macos")]
        0x00070052 => Key::Other(126),
        #[cfg(target_os = "windows")]
        0x00070052 => Key::Other(57416),
        // NumLock
        #[cfg(target_os = "macos")]
        0x00070053 => Key::Other(71),
        #[cfg(target_os = "windows")]
        0x00070053 => Key::Other(57413),
        // NumpadDivide
        #[cfg(target_os = "macos")]
        0x00070054 => Key::Other(75),
        #[cfg(target_os = "windows")]
        0x00070054 => Key::Other(57397),
        // NumpadMultiply
        #[cfg(target_os = "macos")]
        0x00070055 => Key::Other(67),
        #[cfg(target_os = "windows")]
        0x00070055 => Key::Other(55),
        // NumpadSubtract
        #[cfg(target_os = "macos")]
        0x00070056 => Key::Other(78),
        #[cfg(target_os = "windows")]
        0x00070056 => Key::Other(74),
        // NumpadAdd
        #[cfg(target_os = "macos")]
        0x00070057 => Key::Other(69),
        #[cfg(target_os = "windows")]
        0x00070057 => Key::Other(78),
        // NumpadEnter
        #[cfg(target_os = "macos")]
        0x00070058 => Key::Other(76),
        #[cfg(target_os = "windows")]
        0x00070058 => Key::Other(57372),
        // Numpad1
        #[cfg(target_os = "macos")]
        0x00070059 => Key::Other(83),
        #[cfg(target_os = "windows")]
        0x00070059 => Key::Other(79),
        // Numpad2
        #[cfg(target_os = "macos")]
        0x0007005a => Key::Other(84),
        #[cfg(target_os = "windows")]
        0x0007005a => Key::Other(80),
        // Numpad3
        #[cfg(target_os = "macos")]
        0x0007005b => Key::Other(85),
        #[cfg(target_os = "windows")]
        0x0007005b => Key::Other(81),
        // Numpad4
        #[cfg(target_os = "macos")]
        0x0007005c => Key::Other(86),
        #[cfg(target_os = "windows")]
        0x0007005c => Key::Other(75),
        // Numpad5
        #[cfg(target_os = "macos")]
        0x0007005d => Key::Other(87),
        #[cfg(target_os = "windows")]
        0x0007005d => Key::Other(76),
        // Numpad6
        #[cfg(target_os = "macos")]
        0x0007005e => Key::Other(88),
        #[cfg(target_os = "windows")]
        0x0007005e => Key::Other(77),
        // Numpad7
        #[cfg(target_os = "macos")]
        0x0007005f => Key::Other(89),
        #[cfg(target_os = "windows")]
        0x0007005f => Key::Other(71),
        // Numpad8
        #[cfg(target_os = "macos")]
        0x00070060 => Key::Other(91),
        #[cfg(target_os = "windows")]
        0x00070060 => Key::Other(72),
        // Numpad9
        #[cfg(target_os = "macos")]
        0x00070061 => Key::Other(92),
        #[cfg(target_os = "windows")]
        0x00070061 => Key::Other(73),
        // Numpad0
        0x00070062 => Key::Other(82),
        // NumpadDecimal
        #[cfg(target_os = "macos")]
        0x00070063 => Key::Other(65),
        #[cfg(target_os = "windows")]
        0x00070063 => Key::Other(83),
        // IntlBackslash
        #[cfg(target_os = "macos")]
        0x00070064 => Key::Other(10),
        #[cfg(target_os = "windows")]
        0x00070064 => Key::Other(86),
        // ContextMenu
        #[cfg(target_os = "macos")]
        0x00070065 => Key::Other(110),
        #[cfg(target_os = "windows")]
        0x00070065 => Key::Other(57437),
        // Power
        #[cfg(target_os = "windows")]
        0x00070066 => Key::Other(57438),
        // NumpadEqual
        #[cfg(target_os = "macos")]
        0x00070067 => Key::Other(81),
        #[cfg(target_os = "windows")]
        0x00070067 => Key::Other(89),
        // F13
        #[cfg(target_os = "macos")]
        0x00070068 => Key::Other(105),
        #[cfg(target_os = "windows")]
        0x00070068 => Key::Other(100),
        // F14
        #[cfg(target_os = "macos")]
        0x00070069 => Key::Other(107),
        #[cfg(target_os = "windows")]
        0x00070069 => Key::Other(101),
        // F15
        #[cfg(target_os = "macos")]
        0x0007006a => Key::Other(113),
        #[cfg(target_os = "windows")]
        0x0007006a => Key::Other(102),
        // F16
        #[cfg(target_os = "macos")]
        0x0007006b => Key::Other(106),
        #[cfg(target_os = "windows")]
        0x0007006b => Key::Other(103),
        // F17
        #[cfg(target_os = "macos")]
        0x0007006c => Key::Other(64),
        #[cfg(target_os = "windows")]
        0x0007006c => Key::Other(104),
        // F18
        #[cfg(target_os = "macos")]
        0x0007006d => Key::Other(79),
        #[cfg(target_os = "windows")]
        0x0007006d => Key::Other(105),
        // F19
        #[cfg(target_os = "macos")]
        0x0007006e => Key::Other(80),
        #[cfg(target_os = "windows")]
        0x0007006e => Key::Other(106),
        // F20
        #[cfg(target_os = "macos")]
        0x0007006f => Key::Other(90),
        #[cfg(target_os = "windows")]
        0x0007006f => Key::Other(107),
        // F21
        #[cfg(target_os = "windows")]
        0x00070070 => Key::Other(108),
        // F22
        #[cfg(target_os = "windows")]
        0x00070071 => Key::Other(109),
        // F23
        #[cfg(target_os = "windows")]
        0x00070072 => Key::Other(110),
        // F24
        #[cfg(target_os = "windows")]
        0x00070073 => Key::Other(118),
        // Help
        #[cfg(target_os = "windows")]
        0x00070075 => Key::Other(57403),
        // Undo
        #[cfg(target_os = "windows")]
        0x0007007a => Key::Other(57352),
        // Cut
        #[cfg(target_os = "windows")]
        0x0007007b => Key::Other(57367),
        // Copy
        #[cfg(target_os = "windows")]
        0x0007007c => Key::Other(57368),
        // Paste
        #[cfg(target_os = "windows")]
        0x0007007d => Key::Other(57354),
        // NumpadComma
        #[cfg(target_os = "macos")]
        0x00070085 => Key::Other(95),
        #[cfg(target_os = "windows")]
        0x00070085 => Key::Other(126),
        // IntlRo
        #[cfg(target_os = "macos")]
        0x00070087 => Key::Other(94),
        #[cfg(target_os = "windows")]
        0x00070087 => Key::Other(115),
        // KanaMode
        #[cfg(target_os = "windows")]
        0x00070088 => Key::Other(112),
        // IntlYen
        #[cfg(target_os = "macos")]
        0x00070089 => Key::Other(93),
        #[cfg(target_os = "windows")]
        0x00070089 => Key::Other(125),
        // Convert
        #[cfg(target_os = "windows")]
        0x0007008a => Key::Other(121),
        // NonConvert
        #[cfg(target_os = "windows")]
        0x0007008b => Key::Other(123),
        // Lang1
        #[cfg(target_os = "macos")]
        0x00070090 => Key::Other(104),
        #[cfg(target_os = "windows")]
        0x00070090 => Key::Other(114),
        // Lang2
        #[cfg(target_os = "macos")]
        0x00070091 => Key::Other(102),
        #[cfg(target_os = "windows")]
        0x00070091 => Key::Other(113),
        // Lang3
        #[cfg(target_os = "windows")]
        0x00070092 => Key::Other(120),
        // Lang4
        #[cfg(target_os = "windows")]
        0x00070093 => Key::Other(119),
        // ControlLeft
        #[cfg(target_os = "macos")]
        0x000700e0 => Key::Other(59),
        #[cfg(target_os = "windows")]
        0x000700e0 => Key::Other(29),
        // ShiftLeft
        #[cfg(target_os = "macos")]
        0x000700e1 => Key::Other(56),
        #[cfg(target_os = "windows")]
        0x000700e1 => Key::Other(42),
        // AltLeft
        #[cfg(target_os = "macos")]
        0x000700e2 => Key::Other(58),
        #[cfg(target_os = "windows")]
        0x000700e2 => Key::Other(56),
        // MetaLeft
        #[cfg(target_os = "macos")]
        0x000700e3 => Key::Other(55),
        #[cfg(target_os = "windows")]
        0x000700e3 => Key::Other(57435),
        // ControlRight
        #[cfg(target_os = "macos")]
        0x000700e4 => Key::Other(62),
        #[cfg(target_os = "windows")]
        0x000700e4 => Key::Other(57373),
        // ShiftRight
        #[cfg(target_os = "macos")]
        0x000700e5 => Key::Other(60),
        #[cfg(target_os = "windows")]
        0x000700e5 => Key::Other(54),
        // AltRight
        #[cfg(target_os = "macos")]
        0x000700e6 => Key::Other(61),
        #[cfg(target_os = "windows")]
        0x000700e6 => Key::Other(57400),
        // MetaRight
        #[cfg(target_os = "macos")]
        0x000700e7 => Key::Other(54),
        #[cfg(target_os = "windows")]
        0x000700e7 => Key::Other(57436),
        // MediaStop
        #[cfg(target_os = "windows")]
        0x000c00b7 => Key::Other(57380),
        // Eject
        #[cfg(target_os = "windows")]
        0x000c00b8 => Key::Other(57388),
        // MediaSelect
        #[cfg(target_os = "windows")]
        0x000c0183 => Key::Other(57453),
        // LaunchMail
        #[cfg(target_os = "windows")]
        0x000c018a => Key::Other(57452),
        // LaunchApp2
        #[cfg(target_os = "windows")]
        0x000c0192 => Key::Other(57377),
        // LaunchApp1
        #[cfg(target_os = "windows")]
        0x000c0194 => Key::Other(57451),
        // BrowserSearch
        #[cfg(target_os = "windows")]
        0x000c0221 => Key::Other(57445),
        // BrowserHome
        #[cfg(target_os = "windows")]
        0x000c0223 => Key::Other(57394),
        // BrowserBack
        #[cfg(target_os = "windows")]
        0x000c0224 => Key::Other(57450),
        // BrowserForward
        #[cfg(target_os = "windows")]
        0x000c0225 => Key::Other(57449),
        // BrowserStop
        #[cfg(target_os = "windows")]
        0x000c0226 => Key::Other(57448),
        // BrowserRefresh
        #[cfg(target_os = "windows")]
        0x000c0227 => Key::Other(57447),
        // BrowserFavorites
        #[cfg(target_os = "windows")]
        0x000c022a => Key::Other(57446),

        // 需要特殊处理
        0x00070080 => Key::VolumeUp,
        0x00070081 => Key::VolumeDown,
        0x0007007f => Key::VolumeMute,
        #[cfg(target_os = "macos")]
        0x000c0079 => Key::BrightnessUp,
        #[cfg(target_os = "macos")]
        0x000c007a => Key::BrightnessDown,
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
        0x000c006f => Key::IlluminationUp,
        #[cfg(target_os = "macos")]
        0x000c0070 => Key::IlluminationDown,
        #[cfg(target_os = "macos")]
        0x000c0072 => Key::IlluminationToggle,

        _ => Key::Other(0),
    }
}

#[frb(init)]
pub fn init_app() {
    setup_default_user_utils();
}
