//! Module containing a concrete implementation of SystemEventFacade
//! using OpenGL (gl crate) and GLFW (glfw crate)

use crate::glfw::{Action, Context, Key};

use crate::timer::Timer;
use crate::KeyCode;
use crate::{error::GameResult, gl, glfw, input::Input, timer::GetTime, window::Window};
use glfw::{Monitor, MouseButton, WindowEvent, WindowMode};

use super::SystemEventFacade;

pub struct GLFWBackend {
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
    primary_monitor: glfw::Monitor,
    // fullscreen management
    windowed_width: usize,
    windowed_height: usize,
}

impl GLFWBackend {
    /// Initializes and configures window exactly as in given reference
    pub fn new(window: &Window) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let primary_monitor = Monitor::from_primary();
        let mode = WindowMode::Windowed;
        if window.is_fullscreen() {
            window.set_fullscreen(true);
        }

        let result = glfw.create_window(
            window.width() as u32,
            window.height() as u32,
            window.name(),
            mode,
        );

        match result {
            Some((mut w, events)) => {
                w.set_pos(window.pos().0 as i32, window.pos().1 as i32);
                gl::load_with(|s| w.get_proc_address(s) as *const _);
                unsafe {
                    gl::Enable(gl::BLEND);
                    gl::Enable(gl::DEPTH_TEST);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                    gl::Viewport(0, 0, window.width() as i32, window.height() as i32);
                }
                w.set_key_polling(true);
                w.make_current();
                GLFWBackend {
                    window: w,
                    events,
                    glfw,
                    primary_monitor,
                    windowed_width: window.width(),
                    windowed_height: window.height(),
                }
            }
            None => panic!("Failed to create GLFW Window"),
        }
    }
}

impl GetTime for glfw::Glfw {
    fn get_timestamp(&self) -> f64 {
        self.get_time()
    }
}

impl SystemEventFacade for GLFWBackend {
    fn loop_start(
        &mut self,
        window: &mut Window,
        input: &mut Input,
        timer: &mut Timer,
    ) -> GameResult {
        timer.loop_start(&self.glfw);
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                // Keyboard events
                WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeySpace)
                }
                WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeySpace)
                }
                WindowEvent::Key(Key::Apostrophe, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyApostrophe)
                }
                WindowEvent::Key(Key::Apostrophe, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyApostrophe)
                }
                WindowEvent::Key(Key::Comma, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyComma)
                }
                WindowEvent::Key(Key::Comma, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyComma)
                }
                WindowEvent::Key(Key::Minus, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyMinus)
                }
                WindowEvent::Key(Key::Minus, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyMinus)
                }
                WindowEvent::Key(Key::Period, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyPeriod)
                }
                WindowEvent::Key(Key::Period, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyPeriod)
                }
                WindowEvent::Key(Key::Slash, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeySlash)
                }
                WindowEvent::Key(Key::Slash, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeySlash)
                }
                WindowEvent::Key(Key::Num0, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::Key0)
                }
                WindowEvent::Key(Key::Num0, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::Key0)
                }
                WindowEvent::Key(Key::Num1, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::Key1)
                }
                WindowEvent::Key(Key::Num1, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::Key1)
                }
                WindowEvent::Key(Key::Num2, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::Key2)
                }
                WindowEvent::Key(Key::Num2, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::Key2)
                }
                WindowEvent::Key(Key::Num3, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::Key3)
                }
                WindowEvent::Key(Key::Num3, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::Key3)
                }
                WindowEvent::Key(Key::Num4, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::Key4)
                }
                WindowEvent::Key(Key::Num4, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::Key4)
                }
                WindowEvent::Key(Key::Num5, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::Key5)
                }
                WindowEvent::Key(Key::Num5, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::Key5)
                }
                WindowEvent::Key(Key::Num6, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::Key6)
                }
                WindowEvent::Key(Key::Num6, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::Key6)
                }
                WindowEvent::Key(Key::Num7, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::Key7)
                }
                WindowEvent::Key(Key::Num7, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::Key7)
                }
                WindowEvent::Key(Key::Num8, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::Key8)
                }
                WindowEvent::Key(Key::Num8, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::Key8)
                }
                WindowEvent::Key(Key::Num9, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::Key9)
                }
                WindowEvent::Key(Key::Num9, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::Key9)
                }
                WindowEvent::Key(Key::Semicolon, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeySemicolon)
                }
                WindowEvent::Key(Key::Semicolon, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeySemicolon)
                }
                WindowEvent::Key(Key::Equal, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyEqual)
                }
                WindowEvent::Key(Key::Equal, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyEqual)
                }
                WindowEvent::Key(Key::A, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyA),
                WindowEvent::Key(Key::A, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyA)
                }
                WindowEvent::Key(Key::B, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyB),
                WindowEvent::Key(Key::B, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyB)
                }
                WindowEvent::Key(Key::C, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyC),
                WindowEvent::Key(Key::C, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyC)
                }
                WindowEvent::Key(Key::D, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyD),
                WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyD)
                }
                WindowEvent::Key(Key::E, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyE),
                WindowEvent::Key(Key::E, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyE)
                }
                WindowEvent::Key(Key::F, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyF),
                WindowEvent::Key(Key::F, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF)
                }
                WindowEvent::Key(Key::G, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyG),
                WindowEvent::Key(Key::G, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyG)
                }
                WindowEvent::Key(Key::H, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyH),
                WindowEvent::Key(Key::H, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyH)
                }
                WindowEvent::Key(Key::I, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyI),
                WindowEvent::Key(Key::I, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyI)
                }
                WindowEvent::Key(Key::J, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyJ),
                WindowEvent::Key(Key::J, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyJ)
                }
                WindowEvent::Key(Key::K, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyK),
                WindowEvent::Key(Key::K, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyK)
                }
                WindowEvent::Key(Key::L, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyL),
                WindowEvent::Key(Key::L, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyL)
                }
                WindowEvent::Key(Key::M, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyM),
                WindowEvent::Key(Key::M, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyM)
                }
                WindowEvent::Key(Key::N, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyN),
                WindowEvent::Key(Key::N, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyN)
                }
                WindowEvent::Key(Key::O, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyO),
                WindowEvent::Key(Key::O, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyO)
                }
                WindowEvent::Key(Key::P, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyP),
                WindowEvent::Key(Key::P, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyP)
                }
                WindowEvent::Key(Key::Q, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyQ),
                WindowEvent::Key(Key::Q, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyQ)
                }
                WindowEvent::Key(Key::R, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyR),
                WindowEvent::Key(Key::R, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyR)
                }
                WindowEvent::Key(Key::S, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyS),
                WindowEvent::Key(Key::S, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyS)
                }
                WindowEvent::Key(Key::T, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyT),
                WindowEvent::Key(Key::T, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyT)
                }
                WindowEvent::Key(Key::U, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyU),
                WindowEvent::Key(Key::U, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyU)
                }
                WindowEvent::Key(Key::V, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyV),
                WindowEvent::Key(Key::V, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyV)
                }
                WindowEvent::Key(Key::W, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyW),
                WindowEvent::Key(Key::W, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyW)
                }
                WindowEvent::Key(Key::X, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyX),
                WindowEvent::Key(Key::X, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyX)
                }
                WindowEvent::Key(Key::Y, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyY),
                WindowEvent::Key(Key::Y, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyY)
                }
                WindowEvent::Key(Key::Z, _, Action::Press, _) => input.kb.press_key(KeyCode::KeyZ),
                WindowEvent::Key(Key::Z, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyZ)
                }
                WindowEvent::Key(Key::LeftBracket, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyLeftBracket)
                }
                WindowEvent::Key(Key::LeftBracket, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyLeftBracket)
                }
                WindowEvent::Key(Key::Backslash, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyBackslash)
                }
                WindowEvent::Key(Key::Backslash, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyBackslash)
                }
                WindowEvent::Key(Key::RightBracket, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyRightBracket)
                }
                WindowEvent::Key(Key::RightBracket, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyRightBracket)
                }
                WindowEvent::Key(Key::GraveAccent, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyGraveAccent)
                }
                WindowEvent::Key(Key::GraveAccent, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyGraveAccent)
                }
                WindowEvent::Key(Key::World1, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyWorld1)
                }
                WindowEvent::Key(Key::World1, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyWorld1)
                }
                WindowEvent::Key(Key::World2, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyWorld2)
                }
                WindowEvent::Key(Key::World2, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyWorld2)
                }
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyEscape)
                }
                WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyEscape)
                }
                WindowEvent::Key(Key::Enter, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyEnter)
                }
                WindowEvent::Key(Key::Enter, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyEnter)
                }
                WindowEvent::Key(Key::Tab, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyTab)
                }
                WindowEvent::Key(Key::Tab, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyTab)
                }
                WindowEvent::Key(Key::Backspace, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyBackspace)
                }
                WindowEvent::Key(Key::Backspace, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyBackspace)
                }
                WindowEvent::Key(Key::Insert, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyInsert)
                }
                WindowEvent::Key(Key::Insert, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyInsert)
                }
                WindowEvent::Key(Key::Delete, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyDelete)
                }
                WindowEvent::Key(Key::Delete, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyDelete)
                }
                WindowEvent::Key(Key::Right, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyRight)
                }
                WindowEvent::Key(Key::Right, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyRight)
                }
                WindowEvent::Key(Key::Left, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyLeft)
                }
                WindowEvent::Key(Key::Left, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyLeft)
                }
                WindowEvent::Key(Key::Down, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyDown)
                }
                WindowEvent::Key(Key::Down, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyDown)
                }
                WindowEvent::Key(Key::Up, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyUp)
                }
                WindowEvent::Key(Key::Up, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyUp)
                }
                WindowEvent::Key(Key::PageUp, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyPageUp)
                }
                WindowEvent::Key(Key::PageUp, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyPageUp)
                }
                WindowEvent::Key(Key::PageDown, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyPageDown)
                }
                WindowEvent::Key(Key::PageDown, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyPageDown)
                }
                WindowEvent::Key(Key::Home, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyHome)
                }
                WindowEvent::Key(Key::Home, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyHome)
                }
                WindowEvent::Key(Key::End, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyEnd)
                }
                WindowEvent::Key(Key::End, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyEnd)
                }
                WindowEvent::Key(Key::CapsLock, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyCapsLock)
                }
                WindowEvent::Key(Key::CapsLock, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyCapsLock)
                }
                WindowEvent::Key(Key::ScrollLock, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyScrollLock)
                }
                WindowEvent::Key(Key::ScrollLock, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyScrollLock)
                }
                WindowEvent::Key(Key::NumLock, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyNumLock)
                }
                WindowEvent::Key(Key::NumLock, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyNumLock)
                }
                WindowEvent::Key(Key::PrintScreen, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyPrintScreen)
                }
                WindowEvent::Key(Key::PrintScreen, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyPrintScreen)
                }
                WindowEvent::Key(Key::Pause, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyPause)
                }
                WindowEvent::Key(Key::Pause, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyPause)
                }
                WindowEvent::Key(Key::F1, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF1)
                }
                WindowEvent::Key(Key::F1, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF1)
                }
                WindowEvent::Key(Key::F2, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF2)
                }
                WindowEvent::Key(Key::F2, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF2)
                }
                WindowEvent::Key(Key::F3, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF3)
                }
                WindowEvent::Key(Key::F3, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF3)
                }
                WindowEvent::Key(Key::F4, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF4)
                }
                WindowEvent::Key(Key::F4, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF4)
                }
                WindowEvent::Key(Key::F5, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF5)
                }
                WindowEvent::Key(Key::F5, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF5)
                }
                WindowEvent::Key(Key::F6, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF6)
                }
                WindowEvent::Key(Key::F6, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF6)
                }
                WindowEvent::Key(Key::F7, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF7)
                }
                WindowEvent::Key(Key::F7, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF7)
                }
                WindowEvent::Key(Key::F8, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF8)
                }
                WindowEvent::Key(Key::F8, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF8)
                }
                WindowEvent::Key(Key::F9, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF9)
                }
                WindowEvent::Key(Key::F9, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF9)
                }
                WindowEvent::Key(Key::F10, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF10)
                }
                WindowEvent::Key(Key::F10, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF10)
                }
                WindowEvent::Key(Key::F11, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF11)
                }
                WindowEvent::Key(Key::F11, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF11)
                }
                WindowEvent::Key(Key::F12, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF12)
                }
                WindowEvent::Key(Key::F12, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF12)
                }
                WindowEvent::Key(Key::F13, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF13)
                }
                WindowEvent::Key(Key::F13, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF13)
                }
                WindowEvent::Key(Key::F14, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF14)
                }
                WindowEvent::Key(Key::F14, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF14)
                }
                WindowEvent::Key(Key::F15, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF15)
                }
                WindowEvent::Key(Key::F15, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF15)
                }
                WindowEvent::Key(Key::F16, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF16)
                }
                WindowEvent::Key(Key::F16, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF16)
                }
                WindowEvent::Key(Key::F17, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF17)
                }
                WindowEvent::Key(Key::F17, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF17)
                }
                WindowEvent::Key(Key::F18, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF18)
                }
                WindowEvent::Key(Key::F18, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF18)
                }
                WindowEvent::Key(Key::F19, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF19)
                }
                WindowEvent::Key(Key::F19, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF19)
                }
                WindowEvent::Key(Key::F20, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF20)
                }
                WindowEvent::Key(Key::F20, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF20)
                }
                WindowEvent::Key(Key::F21, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF21)
                }
                WindowEvent::Key(Key::F21, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF21)
                }
                WindowEvent::Key(Key::F22, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF22)
                }
                WindowEvent::Key(Key::F22, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF22)
                }
                WindowEvent::Key(Key::F23, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF23)
                }
                WindowEvent::Key(Key::F23, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF23)
                }
                WindowEvent::Key(Key::F24, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF24)
                }
                WindowEvent::Key(Key::F24, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF24)
                }
                WindowEvent::Key(Key::F25, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyF25)
                }
                WindowEvent::Key(Key::F25, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyF25)
                }
                WindowEvent::Key(Key::Kp0, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKp0)
                }
                WindowEvent::Key(Key::Kp0, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKp0)
                }
                WindowEvent::Key(Key::Kp1, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKp1)
                }
                WindowEvent::Key(Key::Kp1, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKp1)
                }
                WindowEvent::Key(Key::Kp2, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKp2)
                }
                WindowEvent::Key(Key::Kp2, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKp2)
                }
                WindowEvent::Key(Key::Kp3, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKp3)
                }
                WindowEvent::Key(Key::Kp3, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKp3)
                }
                WindowEvent::Key(Key::Kp4, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKp4)
                }
                WindowEvent::Key(Key::Kp4, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKp4)
                }
                WindowEvent::Key(Key::Kp5, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKp5)
                }
                WindowEvent::Key(Key::Kp5, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKp5)
                }
                WindowEvent::Key(Key::Kp6, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKp6)
                }
                WindowEvent::Key(Key::Kp6, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKp6)
                }
                WindowEvent::Key(Key::Kp7, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKp7)
                }
                WindowEvent::Key(Key::Kp7, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKp7)
                }
                WindowEvent::Key(Key::Kp8, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKp8)
                }
                WindowEvent::Key(Key::Kp8, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKp8)
                }
                WindowEvent::Key(Key::Kp9, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKp9)
                }
                WindowEvent::Key(Key::Kp9, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKp9)
                }
                WindowEvent::Key(Key::KpDecimal, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKpDecimal)
                }
                WindowEvent::Key(Key::KpDecimal, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKpDecimal)
                }
                WindowEvent::Key(Key::KpDivide, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKpDivide)
                }
                WindowEvent::Key(Key::KpDivide, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKpDivide)
                }
                WindowEvent::Key(Key::KpMultiply, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKpMultiply)
                }
                WindowEvent::Key(Key::KpMultiply, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKpMultiply)
                }
                WindowEvent::Key(Key::KpSubtract, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKpSubtract)
                }
                WindowEvent::Key(Key::KpSubtract, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKpSubtract)
                }
                WindowEvent::Key(Key::KpAdd, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKpAdd)
                }
                WindowEvent::Key(Key::KpAdd, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKpAdd)
                }
                WindowEvent::Key(Key::KpEnter, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKpEnter)
                }
                WindowEvent::Key(Key::KpEnter, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKpEnter)
                }
                WindowEvent::Key(Key::KpEqual, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyKpEqual)
                }
                WindowEvent::Key(Key::KpEqual, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyKpEqual)
                }
                WindowEvent::Key(Key::LeftShift, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyLeftShift)
                }
                WindowEvent::Key(Key::LeftShift, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyLeftShift)
                }
                WindowEvent::Key(Key::LeftControl, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyLeftControl)
                }
                WindowEvent::Key(Key::LeftControl, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyLeftControl)
                }
                WindowEvent::Key(Key::LeftAlt, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyLeftAlt)
                }
                WindowEvent::Key(Key::LeftAlt, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyLeftAlt)
                }
                WindowEvent::Key(Key::LeftSuper, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyLeftSuper)
                }
                WindowEvent::Key(Key::LeftSuper, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyLeftSuper)
                }
                WindowEvent::Key(Key::RightShift, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyRightShift)
                }
                WindowEvent::Key(Key::RightShift, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyRightShift)
                }
                WindowEvent::Key(Key::RightControl, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyRightControl)
                }
                WindowEvent::Key(Key::RightControl, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyRightControl)
                }
                WindowEvent::Key(Key::RightAlt, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyRightAlt)
                }
                WindowEvent::Key(Key::RightAlt, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyRightAlt)
                }
                WindowEvent::Key(Key::RightSuper, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyRightSuper)
                }
                WindowEvent::Key(Key::RightSuper, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyRightSuper)
                }
                WindowEvent::Key(Key::Menu, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyMenu)
                }
                WindowEvent::Key(Key::Menu, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyMenu)
                }
                WindowEvent::Key(Key::Unknown, _, Action::Press, _) => {
                    input.kb.press_key(KeyCode::KeyUnknown)
                }
                WindowEvent::Key(Key::Unknown, _, Action::Release, _) => {
                    input.kb.release_key(KeyCode::KeyUnknown)
                }
                _ => (),
            }
        }
        // Somehow window events are ignored and we need to check them manually
        {
            let (w, h) = self.window.get_framebuffer_size();
            if (w as usize, h as usize) != (window.width(), window.height())
                && !window.is_fullscreen
            {
                window.system_update_resolution(w as usize, h as usize);
                unsafe {
                    gl::Viewport(0, 0, w, h);
                }
            }
            let (x, y) = self.window.get_pos();
            if (x as isize, y as isize) != window.pos() && !window.is_fullscreen {
                window.system_set_pos(x as isize, y as isize);
            }
        }
        // The same goes for Mouse input data
        {
            let (x, y) = self.window.get_cursor_pos();
            let (x, y) = (x as f32, y as f32);
            let (old_x, old_y) = input.mouse.position;
            input.mouse.position = (x, y);
            input.mouse.position_delta = (x - old_x, y - old_y);

            let left_btn = self.window.get_mouse_button(MouseButton::Button1);
            let right_btn = self.window.get_mouse_button(MouseButton::Button2);
            let middle_btn = self.window.get_mouse_button(MouseButton::Button3);
            match left_btn {
                Action::Press => input
                    .mouse
                    .press_button(crate::input::mouse::MouseButton::Left),
                Action::Release => input
                    .mouse
                    .release_button(crate::input::mouse::MouseButton::Left),
                _ => (),
            }
            match right_btn {
                Action::Press => input
                    .mouse
                    .press_button(crate::input::mouse::MouseButton::Right),
                Action::Release => input
                    .mouse
                    .release_button(crate::input::mouse::MouseButton::Right),
                _ => (),
            }
            match middle_btn {
                Action::Press => input
                    .mouse
                    .press_button(crate::input::mouse::MouseButton::Middle),
                Action::Release => input
                    .mouse
                    .release_button(crate::input::mouse::MouseButton::Middle),
                _ => (),
            }
        }
        // TODO! Mouse scroll

        Ok(())
    }

    fn loop_end(
        &mut self,
        window: &mut Window,
        input: &mut Input,
        timer: &mut Timer,
    ) -> GameResult {
        // Input changes handling
        {
            // TODO! Move this line to input update?
            input.mouse.scroll_delta = (0.0, 0.0);
            if input.mouse.is_cursor_visible() {
                self.window.set_cursor_mode(glfw::CursorMode::Normal);
            } else {
                self.window.set_cursor_mode(glfw::CursorMode::Disabled);
            }
        }
        // Window changes handling
        self.window.swap_buffers();
        {
            // fullscreen handling
            let mut fullscreen: bool = false;
            self.window.with_window_mode(|mode| match mode {
                WindowMode::FullScreen(_) => fullscreen = true,
                _ => (),
            });
            if *window.fullscreen_requested.borrow() && !fullscreen {
                let vidmode = self.primary_monitor.get_video_mode().unwrap();
                window.is_fullscreen = true;
                // save window size
                self.windowed_width = window.width();
                self.windowed_height = window.height();
                self.window.set_monitor(
                    glfw::WindowMode::FullScreen(&self.primary_monitor),
                    0,
                    0,
                    vidmode.width,
                    vidmode.height,
                    None,
                );
                window.system_update_resolution(vidmode.width as usize, vidmode.height as usize);
                unsafe {
                    gl::Viewport(0, 0, vidmode.width as i32, vidmode.height as i32);
                }
            }
            if !*window.fullscreen_requested.borrow() && fullscreen {
                window.is_fullscreen = false;
                window.system_update_resolution(self.windowed_width, self.windowed_height);
                self.window.set_monitor(
                    glfw::WindowMode::Windowed,
                    window.pos().0 as i32,
                    window.pos().1 as i32,
                    window.width() as u32,
                    window.height() as u32,
                    None,
                );
                unsafe {
                    gl::Viewport(0, 0, window.width() as i32, window.height() as i32);
                }
            }
        }
        if self.window.should_close() {
            window.system_close();
        }
        timer.loop_end(&self.glfw);
        Ok(())
    }
}
