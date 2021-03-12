mod combat_data;
mod content;
mod def;
mod simulation;
mod vars;

use combat_data::{CombatSetup, HeroSetup};
use simulation::Simulation;

fn main() {
    let hero = HeroSetup::new(
        250.0, 5.0, 9.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 10.0, 0.6,
    );
    let setup = CombatSetup::new(1, 0.95, hero, vec!["slime".into(), "slime".into()]);
    let mut simulation = Simulation::new(setup);
    let result = simulation.run();

    println!("Simulation finished, result {:#?}", result);
}
