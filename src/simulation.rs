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
        let secs = self.ticks as f32 / TICKS_PER_SECOND as f32;
        result.duration = secs;
        result.dps = result.actual_dmg_dealt / secs;
        result
    }

    fn tick(&mut self, result: &mut CombatResult) {
        self.hero_attack(result);
        self.monster_attack(result);

        // remove all dead monsters
        self.monsters.retain(|m| !m.is_dead());
        // if all monsters are dead, spawn the next set
        if self.monsters.is_empty() {
            self.respawn_monsters();
            self.hero.atk_tick = 0;
            self.hero.stamina = STAMINA_BASE;
            result.encounters_cleared += 1;
        } else {
            // hero stamina regeneration
            self.hero.stamina += STAMINA_PERSEC / TICKS_PER_SECOND as f32;
            //TODO: that means hero cannot exceed starting stamina, is that correct?
            self.hero.stamina = self.hero.stamina.clamp(0.0, STAMINA_BASE);
            // hero regeneration; cannot exceed max hp
            let regen = self.hero.stats().regen() / TICKS_PER_SECOND as f32;
            self.hero.hp = (self.hero.hp + regen).clamp(0.0, self.hero.stats().max_hp());
        }

        self.ticks += 1;
    }

    fn hero_attack(&mut self, result: &mut CombatResult) {
        self.hero.atk_tick += 1;
        let atk_tick_target = (ATTSPD_BASE * TICKS_PER_SECOND as f32) as i32;
        if self.hero.atk_tick >= atk_tick_target && self.hero.stamina >= STAMINA_ATTACK {
            let dmg_min = self.hero.stats().dmg_min();
            let dmg_max = self.hero.stats().dmg_max();
            let dmg_raw = self.rng.gen_range(dmg_min..=dmg_max);

            let monster_idx = self.get_monster_for_attack();
            {
                let monster = &mut self.monsters[monster_idx];
                //TODO: counter not considered
                let hit_roll: f32 = self.rng.gen_range(0.0..=100.0);
                if hit_roll > monster.stats().evade() {
                    let mut dmg = get_dmg_reduced_by_def(dmg_raw, monster.stats().defense());
                    let dmg_magic = self.hero.stats().magic_dmg();
                    dmg += dmg_magic;
                    monster.hp -= dmg;

                    result.hits_landed += 1;
                    result.actual_dmg_dealt += dmg;
                    result.unmitigated_dmg_dealt += dmg_raw + dmg_magic;
                }
            }
            let dmg_all = self.hero.stats().dmg_all();
            if dmg_all > 0.0 {
                for monster in &mut self.monsters {
                    let dmg = get_dmg_reduced_by_def(dmg_all, monster.stats().defense());

                    monster.hp -= dmg;

                    result.actual_dmg_dealt += dmg;
                    result.unmitigated_dmg_dealt += dmg_all;
                }
            }

            // hero just attacked, so reset attack timer
            self.hero.atk_tick = 0;
            self.hero.stamina -= STAMINA_ATTACK;
        }
    }

    fn monster_attack(&mut self, result: &mut CombatResult) {
        for monster in &mut self.monsters {
            monster.atk_tick += 1;
            let atk_tick_target = (monster.stats().spd() * TICKS_PER_SECOND as f32) as i32;
            // check if monster is ready to attack
            if monster.atk_tick >= atk_tick_target {
                let hit_roll: f32 = self.rng.gen_range(0.0..=100.0);
                // attack is a hit if it is above the evade threshold
                // or if the hero does not have enough stamina left for evade
                if hit_roll > self.hero.stats().evade() || self.hero.stamina < STAMINA_EVADE {
                    let dmg_raw =
                        monster.stats().strength() * self.setup.loop_no() * self.setup.diff_scale();
                    let dmg = get_dmg_reduced_by_def(dmg_raw, self.hero.stats().defense());

                    //TODO: counter not considered
                    self.hero.hp -= dmg;
                    self.hero.stamina += STAMINA_NOEVADE;
                    monster.atk_tick = 0;

                    result.actual_dmg_recv += dmg;
                    result.unmitigated_dmg_recv += dmg_raw;
                } else {
                    self.hero.stamina -= STAMINA_EVADE;
                    result.hits_evaded += 1;
                }
            }

            // monster regeneration; cannot exceed max hp
            let regen = monster.stats().regen() / TICKS_PER_SECOND as f32;
            monster.hp = (monster.hp + regen).clamp(0.0, monster.stats().max_hp());
        }
    }

    fn get_monster_for_attack(&mut self) -> usize {
        self.rng.gen_range(0..self.monsters.len())
    }

    fn respawn_monsters(&mut self) {
        let loop_no = self.setup.loop_no();
        self.monsters = self
            .setup
            .monsters()
            .iter()
            .flat_map(|monster_id| {
                let def = self.content.get_monster(monster_id)?;
                Some(Monster::new(def, loop_no))
            })
            .collect();
    }
}

fn get_dmg_reduced_by_def(dmg: f32, defense: f32) -> f32 {
    // DEV said on Discord:
    // "If the defence is lower than the damage, it's flat reduction
    // But past half the damage be the dragons"
    // so we at least know the simple case is correct
    if dmg > defense {
        return dmg - defense;
    }
    // TODO: completely made-up formula. Actual damage reduction formula for defense is unknown right now
    let actual_dmg = dmg * (0.9_f32.powf(defense / 30.0));
    actual_dmg
}

fn min(v1: f32, v2: f32) -> f32 {
    if v1 < v2 {
        v1
    } else {
        v2
    }
}

fn max(v1: f32, v2: f32) -> f32 {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}
