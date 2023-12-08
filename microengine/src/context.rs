use crate::{
    input::Input,
    timer::Timer,
    window::Window,
};

/// Everything every gameobject should know about, grouped in a single struct
/// Methods with immutable reference to self are meant to be used by gameobjects
pub struct Context {
    pub time: Timer,
    pub window: Window,
    pub input: Input,
}

impl Context {
    pub(crate) fn new(time: Timer, window: Window) -> Self {
        Context {
            time,
            window,
            input: Default::default(),
        }
    }


}

impl Default for Context {
    fn default() -> Self {
        Context::new(Default::default(), Default::default())
    }
}
