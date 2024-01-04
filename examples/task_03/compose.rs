use crate::{
    config::LabirynthConfig,
    gameobjects::{Background, Finish, Labirynth, Player},
    utils::Color,
    MAIN_SCENE,
};
use glm::Vec2;
use microengine::{Game, Scene};

fn create_scene(cfg: LabirynthConfig) -> Scene {
    let main_scene = Scene::new(MAIN_SCENE, 3, 10000, true);

    let tri_size = 1.0 / (cfg.size as f32);
    let start_pos = Vec2::new(-1.0 + tri_size, -1.0 + tri_size);
    let end_pos = Vec2::new(1.0 - tri_size, 1.0 - tri_size);
    let labirynth_id = main_scene.add_gameobject(Labirynth::new(cfg.size, cfg.seed), 1);
    let mut player = Player::new(start_pos, Color::new(0.2, 0.1, 0.7), tri_size);
    player.set_labirynt_id(labirynth_id.unwrap());

    let player_id = main_scene.add_gameobject(player, 1);
    let mut finish = Finish::new(end_pos, tri_size);
    finish.set_player_id(player_id.unwrap());
    _ = main_scene.add_gameobject(finish, 1);
    _ = main_scene.add_gameobject(Background::new(Vec2::new(0.0, 0.0), 2.0), 0);
    main_scene
}

pub fn compose(mut game: Game, cfg: LabirynthConfig) -> Game {
    _ = game.add_scene(create_scene(cfg));
    game
}
