use crate::{
    config::MAIN_SCENE, gameobjects::{player::Player, axis::Axis, bubbles::Bubbles, profiler::Profiler, volcano::Terrain, sun::Sun, light_proxy::LightProxy},
    config::VolcanoConfig,
};
use microengine::{Game, Scene};

pub fn compose(mut game: Game, config: VolcanoConfig) -> Game {
    _ = game.add_scene(create_scene(config, MAIN_SCENE));
    game
}

fn create_scene(config: VolcanoConfig, name: &str) -> Scene {
    let mut main_scene = Scene::new(name, 4, 10000, true);
    add_player(&mut main_scene); 
    add_axis(&mut main_scene);
    add_bubbles(&mut main_scene);
    add_terrain(&mut main_scene);
    add_light_proxy(&mut main_scene);
    add_sun(&mut main_scene);
    add_profiler(&mut main_scene);
    main_scene
}


fn add_player(scene: &mut Scene) {
    let player = Player::new(1.0);
    _ = scene.add_gameobject(player, 1).unwrap()
}

fn add_bubbles(scene: &mut Scene) {
    let bubbles = Bubbles::new(700, 20.0, 0.1, "asdads");
    _ = scene.add_gameobject(bubbles, 2).unwrap()
}

fn add_terrain(scene: &mut Scene) {
    let terrain = Terrain::new();
    _ = scene.add_gameobject(terrain, 1).unwrap()
}
fn add_sun(scene: &mut Scene) {
    let sun = Sun::new(glm::Vec3::new(0.0, -1.0, 0.0), 500.0);
    _ = scene.add_gameobject(sun, 1).unwrap()
}
fn add_light_proxy(scene: &mut Scene) {
    let l_proxy = LightProxy::new();
    _ = scene.add_gameobject(l_proxy, 3).unwrap()
}
fn add_axis(scene: &mut Scene) {
    let axis = Axis::new();
    _ = scene.add_gameobject(axis, 1);
}

fn add_profiler(scene: &mut Scene) {
    let profiler = Profiler::new();
    _ = scene.add_gameobject(profiler, 0);
}
