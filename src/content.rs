use std::collections::HashMap;

use crate::{def::MonsterDef};

pub struct Content {
    monster_defs: HashMap<String, MonsterDef>,
}

impl Content {
    pub fn new() -> Self {
        let mut content = Content {
            monster_defs: HashMap::new(),
        };

        let slime = MonsterDef::new("slime", 13.0, 3.3, 0.0, 0.6, 0.0, 0.0);
        content.monster_defs.insert(slime.id().into(), slime);

        let skeleton = MonsterDef::new("skeleton", 12.0, 9.0, 3.5, 0.3, 0.0, 0.0);
        content.monster_defs.insert(skeleton.id().into(), skeleton);

        content
    }

    pub fn get_monster(&self, id: &str) -> Option<MonsterDef> {
        self.monster_defs
            .get(id)
            .map_or(None, |def| Some(def.clone()))
    }
}
