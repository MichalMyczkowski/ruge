use crate::{
    config::MAIN_SCENE, gameobjects::{player::Player, axis::Axis, maze::Maze},
};
use microengine::{Game, Scene, GameObjectId};

pub fn compose(mut game: Game) -> Game {
    _ = game.add_scene(main_scene());
    game
}

fn main_scene() -> Scene {
    let mut main_scene = Scene::new(MAIN_SCENE, 3, 10000, true);
    let player_id = add_player(&mut main_scene); 
    add_axis(&mut main_scene, player_id);
    add_maze(&mut main_scene, player_id);
    main_scene
}


fn add_player(scene: &mut Scene) -> GameObjectId {
    let player = Player::new();
    scene.add_gameobject(player, 1).unwrap()
}

fn add_axis(scene: &mut Scene, player_id: GameObjectId) {
    let mut axis = Axis::new();
    axis.set_player_id(player_id);
    _ = scene.add_gameobject(axis, 1);
}

fn add_maze(scene: &mut Scene, player_id: GameObjectId) {
    let mut m = Maze::new(13, "asdsadsa".into());
    m.set_player_id(player_id);
    _ = scene.add_gameobject(m, 1);
}
