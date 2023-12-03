//! Scenes are basic building blocks of your game in microengine.
//! They are basically dynamic sets of gameobjects representing what is going on right now

//#[cfg(test)]
//mod tests;
mod idmanager;
use crate::{
    context::Context,
    error::{GameError, GameResult},
    gameobject::{GameObject, GameObjectId},
};
use idmanager::IdManager;
use std::{collections::HashMap, iter};

pub struct Scene {
    /// Each scene name must be unique!
    pub(crate) name: String,
    /// if disposable is set to false Game will keep the scene after it is changed
    pub(crate) disposable: bool,
    layers: usize,
    id_manager: IdManager,
    gameobjects: Vec<HashMap<GameObjectId, Box<dyn GameObject>>>,
    new_gameobjects: Vec<(GameObjectId, Box<dyn GameObject>)>,
    dead_ids: Vec<GameObjectId>,
    // TODO? Disable/Enable gameobjects
}

impl Scene {
    pub fn new(name: &str, layers: usize, max_gameobject_count: usize, disposable: bool) -> Self {
        Scene {
            name: String::from(name),
            layers,
            id_manager: IdManager::new(max_gameobject_count),
            gameobjects: iter::repeat_with(|| HashMap::new()).take(layers).collect(),
            new_gameobjects: Vec::new(),
            dead_ids: Vec::new(),
            disposable,
        }
    }

    /// Returns a reference to gameobject with given id
    /// Reference can't be saved because GameObject could be dropped later,
    /// so use it everytime you need to reference another gameobject
    pub fn gameobject_by_id<T: 'static>(&self, id: &GameObjectId) -> Option<&T> {
        self.gameobjects
            .get(id.layer)?
            .get(id)?
            .as_any()
            .downcast_ref::<T>()
    }

    /// Adds given gameobject to scene and returns its Id.
    /// [!WARNING]
    /// Use it while constructing scene and not while it's running!
    pub fn add_gameobject<T: GameObject + 'static>(
        &mut self,
        gameobject: T,
        layer: usize,
    ) -> GameResult<GameObjectId> {
        if layer >= self.layers {
            Err(GameError::SceneError(
                self.name.clone(),
                format!("Layer does not exist: {}", layer),
            ))
        } else {
            let new_id = self.id_manager.get(layer);
            match new_id {
                Ok(id) => {
                    self.new_gameobjects
                        .push((id.clone(), Box::new(gameobject)));
                    Ok(id)
                }
                Err(_) => Err(GameError::SceneError(
                    self.name.clone(),
                    format!("Attempting to create over max_gameobject_count gameobjects"),
                )),
            }
        }
    }

    /// Method run each time the active scene is set to self
    pub fn start(&mut self, ctx: &Context) -> GameResult {
        for layer in self.gameobjects.iter_mut() {
            for (_, go) in layer.iter_mut() {
                go.start(ctx)?;
            }
        }
        Ok(())
    }

    /// All gameobject methods are being run here in this very method (all but start())
    /// returns true if all gameobjects are finished.
    pub fn run_loop(
        &mut self,
        ctx: &Context,
        fixed_time_steps: usize,
        shutdown: bool,
    ) -> GameResult<bool> {
        // Add all newly created gameobjects
        self.new_gameobjects.drain(..).for_each(|(id, go)| {
            self.gameobjects[id.layer].insert(id, go);
        });

        // run fixed update
        for _ in 0..fixed_time_steps {
            for layer in self.gameobjects.iter_mut() {
                for (_, go) in layer.iter_mut() {
                    go.fixed_update(ctx)?;
                }
            }
        }

        // run updates
        let mut finished = 0;
        let mut all = 0;
        for layer in self.gameobjects.iter_mut() {
            for (id, go) in layer.iter_mut() {
                all += 1;
                if shutdown {
                    finished += go.finished_update(ctx)? as usize;
                } else {
                    go.update(ctx)?;
                }
                if go.is_dead() {
                    self.dead_ids.push(id.clone());
                }
            }
        }

        // Delete all dead gameobjects
        self.dead_ids.iter().for_each(|id| {
            self.gameobjects[id.layer].remove(&id);
            self.id_manager.free(id.clone());
        });
        self.dead_ids.clear();

        // draw gameobjects
        for layer in self.gameobjects.iter_mut() {
            for (_, go) in layer.iter_mut() {
                go.draw(ctx)?;
            }
        }
        Ok(all == finished && shutdown)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene::new("default", 2, 10, true)
    }
}

// TODO! Move this module into scene/tests.rs
#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        rc::{Rc, Weak},
    };

    use super::*;
    use crate::context::Context;

    struct TestGO {
        x: u32,
        y: u32,
    }

    impl TestGO {
        pub fn new(x: u32, y: u32) -> Self {
            TestGO { x, y }
        }
        pub fn x(&self) -> u32 {
            self.x
        }
        pub fn y(&self) -> u32 {
            self.y
        }
    }

    impl GameObject for TestGO {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    fn empty_scene(max_ids: usize) -> Scene {
        Scene::new("test", 3, max_ids, true)
    }

    #[test]
    fn finds_gameobject_by_id() {
        let mut scene = empty_scene(10);
        let ctx = Context::default();
        _ = scene.add_gameobject(TestGO::new(0, 0), 0);
        let (x, y) = (1, 2);
        let id = scene.add_gameobject(TestGO::new(x, y), 0).unwrap();
        _ = scene.run_loop(&ctx, 0, false);
        let returned = scene.gameobject_by_id::<TestGO>(&id).unwrap();
        assert_eq!(returned.x(), x);
        assert_eq!(returned.y(), y);
    }

    #[test]
    fn cant_instantiate_more_than_max_count_gameobjects() {
        let mut scene = empty_scene(1);
        _ = scene.add_gameobject(TestGO::new(0, 0), 0);
        let err = scene.add_gameobject(TestGO::new(0, 0), 0);
        assert_eq!(true, err.is_err());
    }

    struct TestDrop {
        pub dropped: Weak<RefCell<bool>>,
    }

    impl GameObject for TestDrop {
        fn is_dead(&mut self) -> bool {
            true
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl Drop for TestDrop {
        fn drop(&mut self) {
            *self.dropped.upgrade().unwrap().borrow_mut() = true;
        }
    }

    #[test]
    fn gameobjects_are_dropped_if_dead() {
        let is_dropped = Rc::new(RefCell::new(false));
        let td = TestDrop {
            dropped: Rc::downgrade(&is_dropped),
        };
        let mut scene = empty_scene(10);
        let ctx = Context::default();
        _ = scene.add_gameobject(td, 0);
        _ = scene.run_loop(&ctx, 0, false);
        assert_eq!(*is_dropped.borrow(), true);
    }

    struct AlwaysDead;
    impl GameObject for AlwaysDead {
        fn is_dead(&mut self) -> bool {
            true
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    fn unused_gameobjects_ids_are_returned_to_pool() {
        let mut scene = empty_scene(1);
        let ctx = Context::default();
        let dead = AlwaysDead {};
        _ = scene.add_gameobject(dead, 0);
        _ = scene.run_loop(&ctx, 0, false);
        let dead = AlwaysDead {};
        let id = scene.add_gameobject(dead, 0);
        assert_eq!(true, id.is_ok());
    }

    #[test]
    fn returns_true_after_all_gameobjects_are_finished() {
        let mut scene = empty_scene(1);
        let ctx = Context::default();
        // TestGO returns true on finish by deafault
        let dead = TestGO::new(0, 0);
        _ = scene.add_gameobject(dead, 0);
        let finished = scene.run_loop(&ctx, 0, true).unwrap();
        assert_eq!(true, finished);
    }

    struct FixedUpdateCheck(usize);
    impl GameObject for FixedUpdateCheck {
        fn fixed_update(&mut self, _ctx: &Context) -> GameResult {
            self.0 += 1;
            Ok(())
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    fn runs_fixed_update_n_times() {
        let mut scene = empty_scene(1);
        let ctx = Context::default();
        let go = FixedUpdateCheck(0);
        let id = scene.add_gameobject(go, 0).unwrap();
        _ = scene.run_loop(&ctx, 3, false).unwrap();
        {
            let go = scene.gameobject_by_id::<FixedUpdateCheck>(&id).unwrap();
            assert_eq!(3, go.0);
        }
    }
}
