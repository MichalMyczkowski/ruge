use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::*;
use crate::{
    context::Context,
    timer::{GetTime, Timer},
};

struct TestGO {
    x: u32,
    y: u32,
    name: String,
}

impl TestGO {
    pub fn new(x: u32, y: u32) -> Self {
        TestGO { x, y, name: String::from("test") }
    }
    pub fn x(&self) -> u32 {
        self.x
    }
    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
}

impl GameObject for TestGO {
    fn name(&self) -> &str {
        &self.name
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn empty_scene(max_ids: usize) -> Scene {
    Scene::new("test", 3, max_ids, true)
}

#[test]
// TODO! this test has 0 sense as it is performed from scene owner perspective
//fn scene_finds_gameobject_by_id() {
//    assert!(true);
//    //let mut scene = empty_scene(10);
//    //let ctx = Context::default();
//    //_ = scene.add_gameobject(TestGO::new(0, 0), 0);
//    //let (x, y) = (1, 2);
//    //let id = scene.add_gameobject(TestGO::new(x, y), 0).unwrap();
//    //_ = scene.run_loop(&ctx, 0, false);
//    //let returned = scene.gameobject_by_id::<TestGO>(&id).unwrap();
//    //assert_eq!(returned.x(), x);
//    //assert_eq!(returned.y(), y);
//}

#[test]
fn scene_cant_instantiate_more_than_max_count_gameobjects() {
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
fn scene_gameobjects_are_dropped_if_dead() {
    let is_dropped = Rc::new(RefCell::new(false));
    let td = TestDrop {
        dropped: Rc::downgrade(&is_dropped),
    };
    let mut scene = empty_scene(10);
    let mut ctx = Context::default();
    _ = scene.add_gameobject(td, 0);
    _ = scene.run_loop(&mut ctx);
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
fn scene_unused_gameobjects_ids_are_returned_to_pool() {
    let mut scene = empty_scene(1);
    let mut ctx = Context::default();
    let dead = AlwaysDead {};
    _ = scene.add_gameobject(dead, 0);
    _ = scene.run_loop(&mut ctx);
    let dead = AlwaysDead {};
    let id = scene.add_gameobject(dead, 0);
    assert_eq!(true, id.is_ok());
}

struct FixedUpdateCheck(usize);
impl GameObject for FixedUpdateCheck {
    fn fixed_update(&mut self, _ctx: &Context, _scene: &Scene) -> GameResult {
        self.0 += 1;
        Ok(())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct MockGetTime {
    time: RefCell<f64>,
}
impl GetTime for MockGetTime {
    fn get_timestamp(&self) -> f64 {
        *self.time.borrow_mut() += 1.0;
        *self.time.borrow()
    }
}

#[test]
fn scene_runs_fixed_update_n_times() {
    let mut scene = empty_scene(1);
    let mut t = Timer::new(50);
    let gt = MockGetTime {
        time: RefCell::new(0.0),
    };
    let mut ctx = Context::default();
    t.loop_start(&gt);
    t.loop_end(&gt);
    ctx.time = t;
    let go = FixedUpdateCheck(0);
    let id = scene.add_gameobject(go, 0).unwrap();
    _ = scene.run_loop(&mut ctx);
    {
        let go = scene.gameobject_by_id::<FixedUpdateCheck>(&id).unwrap();
        assert_eq!(50, go.0);
    }
}

#[test]
fn scene_finds_gameobject_by_name() {
    let mut scene = empty_scene(10);
    // create gameobjects
    let mut go = TestGO::new(1, 2);
    go.set_name("test1");
    let id_test1 = scene.add_gameobject(go, 0).unwrap();
    let mut go = TestGO::new(3, 4);
    go.set_name("test2");
    let id_test2 = scene.add_gameobject(go, 0).unwrap();
    // run loop
    let mut ctx = Context::default();
    _ = scene.run_loop(&mut ctx);
    let returned_id = scene.get_gameobject_id("test1").unwrap();
    assert_eq!(returned_id.id, id_test1.id);
    let returned_id = scene.get_gameobject_id("test2").unwrap();
    assert_eq!(returned_id.id, id_test2.id);
}

struct SearchingGO {
    looking_for_name: String,
    looking_for_id: GameObjectId,
}
impl GameObject for SearchingGO {
    fn update(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
       let t = scene.get_gameobject_id(&self.looking_for_name); 
       assert!(t.is_some_and(|id| id == self.looking_for_id));
       Ok(())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[test]
fn scene_gameobjects_find_others_by_name() {
    let mut scene = empty_scene(10);
    // create gameobjects
    let mut go = TestGO::new(1, 2);
    let name = String::from("testing");
    go.set_name(&name);
    let id = scene.add_gameobject(go, 0).unwrap();
    let searching = SearchingGO {
        looking_for_name: name,
        looking_for_id: id,
    };
    let _ = scene.add_gameobject(searching, 0).unwrap();
    // run loop
    let mut ctx = Context::default();
    _ = scene.run_loop(&mut ctx);
}
