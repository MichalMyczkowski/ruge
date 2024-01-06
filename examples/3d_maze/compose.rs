use crate::{
    config::MAIN_SCENE, gameobjects::{player::Player, axis::Axis, maze::Maze},
    config::MazeConfig,
};
use microengine::{Game, Scene, GameObjectId};

pub fn compose(mut game: Game, config: MazeConfig) -> Game {
    _ = game.add_scene(main_scene(config));
    game
}

fn main_scene(config: MazeConfig) -> Scene {
    let mut main_scene = Scene::new(MAIN_SCENE, 3, 10000, true);
    add_player(&mut main_scene); 
    add_maze(&mut main_scene, &config);
    add_axis(&mut main_scene);
    main_scene
}


fn add_player(scene: &mut Scene) -> GameObjectId {
    let player = Player::new();
    scene.add_gameobject(player, 1).unwrap()
}

fn add_axis(scene: &mut Scene) {
    let axis = Axis::new();
    _ = scene.add_gameobject(axis, 1);
}

fn add_maze(scene: &mut Scene, config: &MazeConfig) {
    let m = Maze::new(config.size, config.seed.clone());
    _ = scene.add_gameobject(m, 1).unwrap();
}
