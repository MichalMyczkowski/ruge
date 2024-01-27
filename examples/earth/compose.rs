use crate::{
    config::MAIN_SCENE,
    config::EarthConfig,
    gameobjects::{axis::Axis, earth::Earth, player::Player},
};
use microengine::{Game, Scene};

pub fn compose(mut game: Game, config: EarthConfig) -> Game {
    _ = game.add_scene(main_scene(config, MAIN_SCENE));
    game
}

fn main_scene(config: EarthConfig, name: &str) -> Scene {
    let mut main_scene = Scene::new(name, 4, 10000, true);
    add_player(&mut main_scene); 
    add_axis(&mut main_scene);
    add_earth(&mut main_scene);
    main_scene
}

fn add_axis(scene: &mut Scene) {
    let axis = Axis::new();
    _ = scene.add_gameobject(axis, 1);
}
fn add_player(scene: &mut Scene) {
    let player = Player::new();
    _ = scene.add_gameobject(player, 1).unwrap()
}
fn add_earth(scene: &mut Scene) {
    let earth = Earth::new();
    _ = scene.add_gameobject(earth, 1);
}
