use crate::{gameobjects::{camera::add_camera, cube::add_cube}, MAIN_SCENE};
use microengine::{Game, prelude::*};


pub fn compose(mut game: Game) -> Game {
    _ = game.add_scene(create_scene(MAIN_SCENE));
    game
}


fn create_scene(name: &str) -> Scene {
    let mut main_scene = Scene::new(name, 4, 100, true);
    add_cube(&mut main_scene);
    add_camera(&mut main_scene); 
    main_scene
}

