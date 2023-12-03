use microengine::{Game, Scene};
use crate::{config::LabirynthConfig, MAIN_SCENE, gameobjects::{Background, Labirynth}};

pub fn compose(mut game: Game, cfg: LabirynthConfig) -> Game {
    let mut main_scene = Scene::new(MAIN_SCENE, 3, 10, true);
    main_scene.add_gameobject(Background::new(200, 0, 112), 0);
    main_scene.add_gameobject(Labirynth::new(cfg.size), 1);
    game.add_scene(main_scene);
    game
}
