#[derive(Clone)]
pub struct MonsterDef {
    id: String,
    max_hp: f32,
    strength: f32,
    defense: f32,
    spd: f32,
    regen: f32,
}

impl MonsterDef {
    pub fn new<T: Into<String>>(
        id: T,
        max_hp: f32,
        strength: f32,
        defense: f32,
        spd: f32,
        regen: f32,
    ) -> Self {
        MonsterDef {
            id: id.into(),
            max_hp,
            strength,
            defense,
            spd,
            regen,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn max_hp(&self) -> f32 {
        self.max_hp
    }

    pub fn strength(&self) -> f32 {
        self.strength
    }

    pub fn defense(&self) -> f32 {
        self.defense
    }

    pub fn spd(&self) -> f32 {
        self.spd
    }

    pub fn regen(&self) -> f32 {
        self.regen
    }
}
