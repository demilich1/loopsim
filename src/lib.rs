mod combat_data;
mod content;
mod def;
mod simulation;
mod vars;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

use combat_data::{CombatSetup, HeroSetup};
use simulation::Simulation;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn simulate(json: String) -> String {
    let hero = HeroSetup::new(
        250.0, 5.0, 9.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 10.0, 0.6,
    );
    let setup = CombatSetup::new(1, 0.95, hero, vec!["slime".into(), "slime".into()]);
    let mut simulation = Simulation::new(setup);
    let result = simulation.run();

    println!("Simulation finished, result {:#?}", result);

    serde_json::to_string(&result).expect("Error while serializing CombatResult to JSON")
}
