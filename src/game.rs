use bevy::prelude::*;
use rand::prelude::*;

use crate::{combination::Combination, dice::NB_DICES};

#[derive(Resource, Default)]
pub struct Tries(pub u8);

#[derive(Resource)]
pub struct ToBeat {
    pub combination: Combination,
    pub tries: u8,
}

impl ToBeat {
    pub fn roll() -> Self {
        let mut tries = 0;

        let combination = loop {
            tries += 1;

            let mut rng = thread_rng();
            let mut dices = [0; NB_DICES];

            for dice in &mut dices {
                *dice = rng.gen_range(1..=6);
            }

            let c = Combination::get(&mut dices);

            match c {
                Combination::Any(_, _, _) | Combination::Nenette => continue,
                _ => break c,
            }
        };

        Self { combination, tries }
    }
}
