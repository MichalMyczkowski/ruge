use crate::{
    config::MAIN_SCENE, gameobjects::{player::Player, axis::Axis},
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
    main_scene
}


fn add_player(scene: &mut Scene) {
    let player = Player::new();
    _ = scene.add_gameobject(player, 1).unwrap()
}

fn add_axis(scene: &mut Scene) {
    let axis = Axis::new();
    _ = scene.add_gameobject(axis, 1);
}
