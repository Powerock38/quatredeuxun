use std::collections::HashMap;

use rand::distributions::weighted::WeightedIndex;
use rand::distributions::Distribution;
use rand::Rng;

pub type DiceResult = u8;

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
