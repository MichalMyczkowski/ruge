//! Module providing struct used by Scene to manage gameobject IDs
//! It is scene's job to ensure that all dropped gameobjects free their ID

use std::iter;
use crate::gameobject::GameObjectId;

pub(crate) struct IdManager {
    taken: usize,
    id_pool: Vec<usize>,
}

impl IdManager {
    pub fn new(max_ids: usize) -> Self {
        let mut cnt = 0;
        IdManager {
            taken: 0,
            id_pool: iter::repeat_with(|| {
                cnt += 1;
                cnt
            }).take(max_ids).collect(),
        }
    }

    pub fn get(&mut self, layer: usize) -> Result<GameObjectId, &'static str> {
        match self.id_pool.pop() {
            Some(t) => {
                self.taken += 1;
                Ok(GameObjectId {layer, id: t})
            },
            None => Err("run out of GameObjectIds")
        }
    }

    /// return GameObjectIds to the pool
    /// if scene frees Ids properly it should never panic
    pub fn free(&mut self, id: GameObjectId) {
        if self.taken == 0 {
            panic!("Returning unknown GameObjectIds");
        }
        self.taken -= 1;
        self.id_pool.push(id.id);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn each_id_is_unique() {
        let max_ids = 20;
        let im = IdManager::new(max_ids);
        assert_eq!(
            im.id_pool.iter().map(|&x| (x, 0)).collect::<HashMap<usize, usize>>().keys().count(),
            max_ids
        );
    }

    #[test]
    #[should_panic]
    fn freeing_more_ids_panics() {
        let max_ids = 20;
        let mut im = IdManager::new(max_ids);
        im.free(GameObjectId { layer: 0, id: 0 });
    }

    #[test]
    fn cant_take_more_than_max() {
        let max_ids = 1;
        let mut im = IdManager::new(max_ids);
        _ = im.get(0);
        assert_eq!(true, im.get(0).is_err());
    }

    #[test]
    fn can_take_returned_ids() {
        let max_ids = 1;
        let mut im = IdManager::new(max_ids);
        let x = im.get(0).unwrap();
        im.free(x);
        assert_eq!(true, im.get(0).is_ok());
    }
}
