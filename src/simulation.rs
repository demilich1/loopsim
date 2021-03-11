use crate::combat_data::{CombatResult, CombatSetup, Hero, Monster};
use crate::content::Content;
use crate::vars::*;

use rand::prelude::*;

pub struct Simulation {
    setup: CombatSetup,
    content: Content,
    rng: ThreadRng,
    hero: Hero,
    monsters: Vec<Monster>,
    ticks: usize,
}

impl Simulation {
    pub fn new(setup: CombatSetup) -> Self {
        let content = Content::new();
        let hero_setup = setup.hero();
        Simulation {
            setup,
            content,
            rng: thread_rng(),
            hero: Hero::new(hero_setup),
            monsters: Vec::new(),
            ticks: 0,
        }
    }

    pub fn run(&mut self) -> CombatResult {
        let mut result = CombatResult::new();
        self.respawn_monsters();
        loop {
            self.ticks += 1;
            if self.ticks > MAX_TICKS {
                break;
            }
            self.tick(&mut result);
            if self.hero.is_dead() {
                break;
            }
        }
        result
    }

    fn tick(&mut self, result: &mut CombatResult) {
        self.hero.atk_tick += 1;
        let atk_tick_target = (ATTSPD_BASE * TICKS_PER_SECOND as f32) as i32;
        if self.hero.atk_tick >= atk_tick_target && self.hero.stamina >= STAMINA_ATTACK {
            let dmg_min = self.hero.stats().dmg_min();
            let dmg_max = self.hero.stats().dmg_max();
            let dmg = self.rng.gen_range(dmg_min..=dmg_max);

            let monster_idx = self.get_monster_for_attack();
            {
                let monster = &mut self.monsters[monster_idx];
                //TODO: defense not considered
                //TODO: evade not considered
                //TODO: counter not considered
                //TODO: magic damage
                monster.hp -= dmg;
            }
            //TODO: damage to all

            // we just attacked, so reset attack counter
            self.hero.atk_tick = 0;
            self.hero.stamina -= STAMINA_ATTACK;

            result.total_damage_dealt += dmg;
        }

        for monster in &mut self.monsters {
            monster.atk_tick += 1;
            let atk_tick_target = (monster.stats().spd() * TICKS_PER_SECOND as f32) as i32;
            if monster.atk_tick >= atk_tick_target {
                let dmg = monster.stats().strength() * self.setup.loop_no() * self.setup.diff_scale();
                //TODO: defense not considered
                //TODO: evade not considered
                //TODO: counter not considered
                self.hero.hp -= dmg;
                monster.atk_tick = 0;
            }
        }

        self.monsters.retain(|m| !m.is_dead());
        if self.monsters.is_empty() {
            self.respawn_monsters();
            result.encounters_cleared += 1;
        }

        // end of tick updates
        self.hero.stamina += STAMINA_PERSEC / TICKS_PER_SECOND as f32;
        //TODO: that means hero cannot exceed starting stamina, is that correct?
        self.hero.stamina = self.hero.stamina.clamp(0.0, STAMINA_BASE);

        self.ticks += 1;
    }

    fn get_monster_for_attack(&mut self) -> usize {
        self.rng.gen_range(0..self.monsters.len())
    }

    fn respawn_monsters(&mut self) {
        self.monsters = self
            .setup
            .monsters()
            .iter()
            .flat_map(|monster_id| {
                let def = self.content.get_monster(monster_id)?;
                Some(Monster::new(def))
            })
            .collect();
    }
}
