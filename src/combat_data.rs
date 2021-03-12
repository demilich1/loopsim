use crate::def::MonsterDef;
use crate::vars::{EVADE_CAP, VAMPIRISM_CAP};

pub struct CombatSetup {
    loop_no: u16,
    diff_scale: f32,
    hero: HeroSetup,
    monsters: Vec<String>,
}

impl CombatSetup {
    pub fn new(loop_no: u16, diff_scale: f32, hero: HeroSetup, monsters: Vec<String>) -> Self {
        CombatSetup {
            loop_no,
            diff_scale,
            hero,
            monsters,
        }
    }

    pub fn loop_no(&self) -> f32 {
        self.loop_no as f32
    }

    pub fn diff_scale(&self) -> f32 {
        self.diff_scale
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
            vampirism: vampirism.clamp(0.0, VAMPIRISM_CAP),
            evade: evade.clamp(0.0, EVADE_CAP),
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
    pub atk_tick: i32,
}

impl Monster {
    pub fn new(def: MonsterDef, loop_no: f32) -> Self {
        let max_hp = def.max_hp() * loop_no;
        Monster { def, hp: max_hp, atk_tick: 0 }
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }

    pub fn stats(&self) -> &MonsterDef {
        &self.def
    }
}

#[derive(Debug, Default)]
pub struct CombatResult {
    pub duration: f32,
    pub encounters_cleared: u32,
    pub hits_landed: u32,
    pub actual_dmg_dealt: f32,
    pub unmitigated_dmg_dealt: f32,
    pub actual_dmg_recv: f32,
    pub unmitigated_dmg_recv: f32,
    pub hits_evaded: u32,
    pub dps: f32,
}

impl CombatResult {
    pub fn new() -> Self {
        Self::default()
    }
}
