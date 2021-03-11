use crate::def::MonsterDef;

pub struct CombatSetup {
    loop_no: u16,
    hero: HeroSetup,
    monsters: Vec<String>,
}

impl CombatSetup {
    pub fn new(loop_no: u16, hero: HeroSetup, monsters: Vec<String>) -> Self {
        CombatSetup {
            loop_no,
            hero,
            monsters,
        }
    }

    pub fn loop_no(&self) -> f32 {
        self.loop_no as f32
    }

    pub fn hero(&self) -> HeroSetup {
        self.hero.clone()
    }

    pub fn monsters(&self) -> &Vec<String> {
        &self.monsters
    }
}

#[derive(Clone)]
pub struct HeroSetup {
    max_hp: f32,
    dmg_min: f32,
    dmg_max: f32,
    dmg_all: f32,
    magic_dmg: f32,
    counter: f32,
    atk_spd: f32,
    crit_chance: f32,
    crit_dmg: f32,
    defense: f32,
    vampirism: f32,
    evade: f32,
    regen: f32,
}

impl HeroSetup {
    pub fn new(
        max_hp: f32,
        dmg_min: f32,
        dmg_max: f32,
        dmg_all: f32,
        magic_dmg: f32,
        counter: f32,
        atk_spd: f32,
        crit_chance: f32,
        crit_dmg: f32,
        defense: f32,
        vampirism: f32,
        evade: f32,
        regen: f32,
    ) -> Self {
        HeroSetup {
            max_hp,
            dmg_min,
            dmg_max,
            dmg_all,
            magic_dmg,
            counter,
            atk_spd,
            crit_chance,
            crit_dmg,
            defense,
            vampirism,
            evade,
            regen,
        }
    }

    pub fn max_hp(&self) -> f32 {
        self.max_hp
    }

    pub fn dmg_min(&self) -> f32 {
        self.dmg_min
    }

    pub fn dmg_max(&self) -> f32 {
        self.dmg_max
    }

    pub fn dmg_all(&self) -> f32 {
        self.dmg_all
    }

    pub fn magic_dmg(&self) -> f32 {
        self.magic_dmg
    }

    pub fn counter(&self) -> f32 {
        self.counter
    }

    pub fn crit_chance(&self) -> f32 {
        self.crit_chance
    }

    pub fn crit_dmg(&self) -> f32 {
        self.crit_dmg
    }

    pub fn defense(&self) -> f32 {
        self.defense
    }

    pub fn vampirism(&self) -> f32 {
        self.vampirism
    }

    pub fn evade(&self) -> f32 {
        self.evade
    }

    pub fn regen(&self) -> f32 {
        self.regen
    }
}

pub struct Hero {
    setup: HeroSetup,
    pub hp: f32,
    pub stamina: f32,
    pub atk_tick: i32,
}

impl Hero {
    pub fn new(setup: HeroSetup) -> Self {
        let max_hp = setup.max_hp();
        Hero {
            setup,
            hp: max_hp,
            stamina: crate::vars::STAMINA_BASE,
            atk_tick: 0,
        }
    }

    pub fn stats(&self) -> &HeroSetup {
        &self.setup
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }
}

pub struct Monster {
    def: MonsterDef,
    pub hp: f32,
}

impl Monster {
    pub fn new(def: MonsterDef) -> Self {
        let max_hp = def.max_hp();
        Monster { def, hp: max_hp }
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }
}

#[derive(Debug)]
pub struct CombatResult {
    pub encounters_cleared: u32,
    pub total_damage_dealt: f32,
}

impl CombatResult {
    pub fn new() -> Self {
        CombatResult {
            encounters_cleared: 0,
            total_damage_dealt: 0.0,
        }
    }
}
