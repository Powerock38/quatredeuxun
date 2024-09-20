use bevy::prelude::*;
use core::fmt;
use rand::prelude::*;

use crate::{combination::Combination, dice::NB_DICES};

#[derive(Resource, Default)]
pub struct Tries(pub u8);

#[derive(Resource)]
pub struct ToBeat {
    pub combination: Combination,
    pub tries: u8,
    challenge: ToBeatChallenge,
}

enum ToBeatChallenge {
    HigherOrEqual,
    Higher,
    Equal,
}

impl ToBeatChallenge {
    fn random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..3) {
            0 => Self::HigherOrEqual,
            1 => Self::Higher,
            _ => Self::Equal,
        }
    }
}

impl ToBeat {
    pub fn roll() -> Self {
        let mut tries = 0;

        let combination = loop {
            tries += 1;

            let mut rng = thread_rng();
            let mut dices = vec![];

            for _ in 0..NB_DICES {
                dices.push(rng.gen_range(1..=6));
            }

            let c = Combination::get(dices);

            match c {
                Combination::Any(_) => continue,
                _ => break c,
            }
        };

        Self {
            combination,
            tries,
            challenge: ToBeatChallenge::random(),
        }
    }

    pub fn is_won(&self, combination: &Combination) -> bool {
        match self.challenge {
            ToBeatChallenge::HigherOrEqual => *combination >= self.combination,
            ToBeatChallenge::Higher => *combination > self.combination,
            ToBeatChallenge::Equal => *combination == self.combination,
        }
    }
}

impl fmt::Display for ToBeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Score {}:  {}",
            match self.challenge {
                ToBeatChallenge::HigherOrEqual => "higher or equal to",
                ToBeatChallenge::Higher => "higher than",
                ToBeatChallenge::Equal => "exactly",
            },
            self.combination,
        )
    }
}
