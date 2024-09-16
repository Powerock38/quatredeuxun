use bevy::prelude::*;
use rand::distributions::weighted::WeightedIndex;
use rand::distributions::Distribution;
use rand::Rng;
use std::collections::HashMap;

use crate::Combination;

pub type DiceResult = u8;

#[derive(Component)]
pub struct Dice {
    weights: HashMap<DiceResult, f32>,
}

impl Dice {
    pub fn new_equal(size: u8) -> Self {
        let mut faces_proba = HashMap::new();
        let p = 1.0 / size as f32;

        for i in 1..=size {
            faces_proba.insert(i, p);
        }

        Self {
            weights: faces_proba,
        }
    }

    pub fn roll(&self, rng: &mut impl Rng) -> DiceResult {
        let (faces, weights): (Vec<&u8>, Vec<&f32>) = self.weights.iter().unzip();

        let dist = WeightedIndex::new(weights).unwrap();

        *faces[dist.sample(rng)]
    }
}

pub fn play_test() {
    let mut rng = rand::thread_rng();

    let dices = [Dice::new_equal(6), Dice::new_equal(6), Dice::new_equal(6)];

    let mut scores = dices
        .iter()
        .map(|dice| dice.roll(&mut rng))
        .collect::<Vec<_>>();

    println!(",___, ,___, ,___,");
    for res in &scores {
        print!("| {} | ", res);
    }
    println!("\n'___' '___' '___'");

    scores.sort_unstable();

    let combination = Combination::get(scores[2], scores[1], scores[0]);

    println!("=> {}  ({} points)", combination, combination.value());
}
