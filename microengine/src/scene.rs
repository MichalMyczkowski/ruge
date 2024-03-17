//! Scenes are basic building blocks of your game in microengine.

pub(crate) mod idmanager;
#[cfg(test)]
mod tests;

use crate::{
    context::Context,
    error::{GameError, GameResult},
    gameobject::{GameObject, GameObjectId},
};
use idmanager::IdManager;
use std::{cell::RefCell, collections::HashMap, iter};

pub struct Scene {
    /// Each scene name must be unique!
    pub(crate) name: String,
    /// if disposable is set to false Game will keep the scene after it is changed
    pub(crate) disposable: bool,
    pub(crate) layers: usize,

    first_loop: bool,
    gameobjects: Vec<HashMap<usize, Option<Box<dyn GameObject>>>>,
    gameobject_ids: Vec<Vec<GameObjectId>>,
    new_gameobjects: RefCell<Vec<(GameObjectId, Box<dyn GameObject>)>>,
    id_manager: RefCell<IdManager>,
    // TODO? Disable/Enable gameobjects
}

impl Scene {
    pub fn new(name: &str, layers: usize, max_gameobject_count: usize, disposable: bool) -> Self {
        Scene {
            name: String::from(name),
            layers,
            first_loop: true, id_manager: RefCell::new(IdManager::new(max_gameobject_count)),
            gameobjects: iter::repeat_with(HashMap::new).take(layers).collect(),
            gameobject_ids: iter::repeat_with(Vec::new).take(layers).collect(),
            new_gameobjects: RefCell::new(Vec::new()),
            disposable,
        }
    }

    /// Returns a reference to gameobject with given id
    /// Reference can't be saved because GameObject could be dropped later,
    /// so use it each time you need to reference another gameobject
    pub fn gameobject_by_id<T: 'static>(&self, id: &GameObjectId) -> Option<&T> {
        // TODO test getting gameobject from different layers
        for layer in self.gameobjects.iter() {
            match layer.get(&id.id) {
                Some(Some(ref go)) => return Some((*go).as_any().downcast_ref::<T>().unwrap()),
                _ => continue,
            }
        }
        None
    }

    /// Returns id of the first gameobject with given name
    /// This method checks each gameobject in scene so it is not recommended to use it
    /// every frame. Instead id should be cached for later use.
    pub fn get_gameobject_id(&self, name: &str) -> Option<GameObjectId> {
        for layer in 0..self.layers {
            for it in 0..self.gameobject_ids[layer].len() {
                let id = self.gameobject_ids[layer][it];
                let go = self.gameobjects[layer].get(&id.id);
                if let Some(go) = go {
                    if go.as_ref().unwrap().name() == name {
                        return Some(id);
                    }
                }
            }
        }
        None
    }

    /// Adds given gameobject to scene and returns its Id.
    pub fn add_gameobject<T: GameObject + 'static>(
        &self,
        gameobject: T,
        layer: usize,
    ) -> GameResult<GameObjectId> {
        if layer >= self.layers {
            Err(GameError::SceneError(
                self.name.clone(),
                format!("Layer does not exist: {}", layer),
            ))
        } else {
            let new_id = self.id_manager.borrow_mut().get(layer)?;
            self.new_gameobjects
                .borrow_mut()
                .push((new_id, Box::new(gameobject)));
            Ok(new_id)
        }
    }

    /// Runs given closure on all gameobjects in scene.
    fn for_all_gameobjects<T>(&mut self, mut f: T) -> GameResult
    where
        T: FnMut(GameObjectId, &mut Box<dyn GameObject>, &Scene) -> GameResult,
    {
        // using for loops instead of mutable iterators
        // so there's no unnecessary mutable reference to scene
        for layer in 0..self.layers {
            for it in 0..self.gameobject_ids[layer].len() {
                let id = self.gameobject_ids[layer][it];
                let go = self.gameobjects[layer].remove(&id.id).unwrap();
                let mut go = match go {
                    Some(g) => g,
                    None => {
                        return Err(GameError::EngineError(
                            "Trying to process a missing gameobject".into(),
                        ))
                    }
                };
                f(id, &mut go, self)?;
                if !go.is_dead() {
                    self.gameobjects[layer].insert(id.id, Some(go));
                } else {
                    self.id_manager.borrow_mut().free(id);
                    self.gameobject_ids[layer][it].is_dead = true;
                }
            }
        }
        Ok(())
    }

    /// All gameobject methods are being run here in this very method
    /// returns true if all gameobjects are finished.
    pub fn run_loop(&mut self, ctx: &mut Context) -> GameResult {
        // add newly created gameobjects
        for (id, mut go) in self.new_gameobjects.borrow_mut().drain(..) {
            if id.layer >= self.layers {
                return Err(
                    GameError::SceneError(self.name.clone(),
                        format!(
                            "Trying to instantiate gameobject to a layer {} that does not exist\n scene.layers: {}",
                            id.layer,
                            self.layers
                        )
                    )
                );
            }
            go.on_add(ctx, self, id)?;
            self.gameobject_ids[id.layer].push(id);
            self.gameobjects[id.layer].insert(id.id, Some(go));
        }
        // run start
        if self.first_loop {
            self.first_loop = false;
            self.for_all_gameobjects(|_, go, scene| go.start(ctx, scene))?;
        }

        // run fixed_update
        for _ in 0..ctx.time.get_fixed_steps() {
            self.for_all_gameobjects(|_, go, scene| go.fixed_update(ctx, scene))?;
        }

        // run update
        self.for_all_gameobjects(|_, go, scene| {
            go.update(ctx, scene)?;
            Ok(())
        })?;

        // delete all dead gameobjects
        self.gameobject_ids.iter_mut().for_each(|v| {
            *v = v.iter().filter(|&id| !id.is_dead ).copied().collect::<Vec<GameObjectId>>();
        });
        
        // draw gameobjects
        self.for_all_gameobjects(|_, go, scene| go.draw(ctx, scene))?;

        Ok(())
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene::new("default", 2, 10, true)
    }
}
