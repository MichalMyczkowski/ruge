use std::cell::RefCell;
use crate::{scene::Scene, timer::Timer, window::Window, input::Input, error::GameResult, gameobject::{GameObject, GameObjectId}};

/// Everything every gameobject should know about, grouped in a single struct
/// Methods with immutable reference to self are meant to be used by gameobjects
pub struct Context {
    /// currently running scene
    /// moved to RefCell so you can push to Scene.new_gameobjects
    pub(crate) scene: Option<RefCell<Scene>>,
    /// vector holding scenes created during runtime
    new_scenes: RefCell<Vec<Scene>>,
    /// Name of scene 
    change_to: RefCell<Option<String>>,
    /// Active scene state
    transitioning: RefCell<bool>,

    pub time: Timer,
    pub window: Window,
    pub input: Input,
}

impl Context {
    pub(crate) fn new(time: Timer, window: Window) -> Self {
        Context {
            scene: None,
            new_scenes: RefCell::new(Vec::new()),
            change_to: RefCell::new(None),
            transitioning: RefCell::new(false),
            time,
            window,
            input: Default::default(),
        }
    }

    /// Consumes given scene and use it as running scene
    /// If there was another scene running before, the ownership will be given back
    pub(crate) fn set_scene(&mut self, mut scene: Scene) -> GameResult<Option<Scene>> {
        // TODO! probably here is a good place to run scene.start
        self.transitioning.replace(false);
        scene.start(&self)?;
        let s = self.scene.replace(RefCell::new(scene));
        match s {
            Some(s) => {
                // TODO? is there a better way to move out of refcell?
                Ok(Some(s.replace(Scene::default())))
            },
            None => Ok(None),
        }
    }

    /// Returns name of a scene that was requested to be changed to
    /// Used by Game to check if it should initiate scene transitioning
    pub(crate) fn scene_should_change(&mut self) -> Option<String> {
        self.change_to.replace(None)
    }
    
    /// Instantiates new gameobjects to currently running scene
    pub fn instantiate<T: GameObject + 'static>(&self, gameobject: T, layer: usize) -> GameResult<GameObjectId> {
        if let Some(ref s) = self.scene {
            s.borrow_mut().add_gameobject(gameobject, layer)
        } else {
            panic!("MicroEngine did not set scene before calling instantiate!");
        }
    }

    /// Use it to transition to a scene pointed to by scene_name string
    /// if active scene was already during transition to another, this method 
    /// will change the transition destination to the scene pointed to by scene_name as it would normally
    /// and return name of a scene it was trying to transition to.
    pub fn change_scene(&self, scene_name: String) -> Option<String> {
        self.transitioning.replace(true);
        self.change_to.replace(Some(scene_name.clone()))
    }

    /// Runs scene loop and indicates if finished
    pub(crate) fn run_current_scene(&mut self) -> GameResult<bool> {
        if let Some(ref scene) = self.scene {
            let fixed_time_steps = self.time.get_fixed_steps();
            let should_finish = *self.transitioning.borrow() || *self.window.close_requested.borrow();
            let finished = scene.borrow_mut().run_loop(&self, fixed_time_steps, should_finish)?;
            if *self.window.close_requested.borrow() && finished {
                self.window.system_close();
            }
            Ok(finished)
        } else {
            panic!("no active scene to call run_loop on");
        }

    }
    // TODO! make it possible to dynamically create scenes from gameobjects

}

impl Default for Context {
    fn default() -> Self {
        Context::new(Default::default(), Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestGO;
    impl GameObject for TestGO {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    #[should_panic]
    fn cant_instantiate_without_active_scene() {
        let c = Context::default();
        let t = TestGO {};
        _ = c.instantiate(t, 0);
    }

    #[test]
    fn consumes_and_returns_scenes() {
        let mut c = Context::default();
        let s = Scene::new("test1", 1, 1, true);
        let ss = Scene::new("test2", 1, 1, true);
        let ret = c.set_scene(s);
        assert!(ret.is_none());
        let ret = c.set_scene(ss);
        assert_eq!(ret.unwrap().name, String::from("test1"));
    }

    #[test]
    fn informs_about_scene_change() {
        let mut c = Context::default();
        { // in some gameobject.update()
           let ctx = &c; 
           ctx.change_scene(String::from("test_scene"));
        }
        assert!(c.scene_should_change().is_some_and(|x| x == "test_scene"));
    }

    #[test]
    fn scene_change_during_transition_changes_destination() {
        let mut c = Context::default();
        { // in some gameobject.update()
           let ctx = &c; 
           ctx.change_scene(String::from("test_scene"));
           let old_name = ctx.change_scene(String::from("test_scene2"));
           assert!(old_name.is_some_and(|x| x == "test_scene"));
        }
        assert!(c.scene_should_change().is_some_and(|x| x == "test_scene2"));
    }
}
