use crate::{
    config::MAIN_SCENE, gameobjects::{player::Player, axis::Axis, bubbles::Bubbles, profiler::Profiler},
    config::VolcanoConfig,
};
use microengine::{Game, Scene};

pub fn compose(mut game: Game, config: VolcanoConfig) -> Game {
    _ = game.add_scene(create_scene(config, MAIN_SCENE));
    game
}

fn create_scene(config: VolcanoConfig, name: &str) -> Scene {
    let mut main_scene = Scene::new(name, 3, 10000, true);
    add_player(&mut main_scene); 
    add_axis(&mut main_scene);
    add_bubbles(&mut main_scene);
    add_profiler(&mut main_scene);
    main_scene
}


fn add_player(scene: &mut Scene) {
    let player = Player::new();
    _ = scene.add_gameobject(player, 1).unwrap()
}

fn add_bubbles(scene: &mut Scene) {
    let bubbles = Bubbles::new(50, 10.0, "asdads");
    _ = scene.add_gameobject(bubbles, 1).unwrap()
}

fn add_axis(scene: &mut Scene) {
    let axis = Axis::new();
    _ = scene.add_gameobject(axis, 1);
}

fn add_profiler(scene: &mut Scene) {
    let profiler = Profiler::new();
    _ = scene.add_gameobject(profiler, 0);
}
