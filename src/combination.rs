use bevy::prelude::*;
use core::fmt;
use std::cmp::Ordering;

use crate::dice::MIN_NB_DICES;

pub type DiceResult = u8;

#[derive(Resource)]
pub struct LastCombination {
    pub combination: Combination,
    pub win: bool,
}

//TODO
// add full house (50/50 with at least a pair of each), 2 pairs, 1 pair, small straight
// Highest roll: > 6 (with special dices)

#[derive(PartialEq, Eq)]
pub enum Combination {
    Any(Vec<DiceResult>),
    LowRoll(Vec<DiceResult>),
    HighRoll(Vec<DiceResult>),
    Straight(DiceResult, usize), // highest dice in the serie, length
    Strike(DiceResult),
    Ace(DiceResult), // can't be 1
    FourTwoOne(usize),
}

impl Combination {
    pub fn get(mut results: Vec<DiceResult>) -> Self {
        assert!(results.len() >= MIN_NB_DICES);

        results.sort_unstable();

        // Check for a "Strike" (all dice are the same)
        if results.iter().all(|&d| d == results[0]) {
            return Combination::Strike(results[0]);
        }

        // Check for an "Ace" (any dice + all 1)
        if results.iter().filter(|&d| *d == 1).count() == results.len() - 1 {
            return Combination::Ace(*results.last().unwrap());
        }

        // Check for a "Straight" (consecutive sequence)
        if results.windows(2).all(|w| w[1] == w[0] + 1) {
            return Combination::Straight(*results.last().unwrap(), results.len());
        }

        // 421
        if results.contains(&4) && results.contains(&2) && results.contains(&1) {
            return Combination::FourTwoOne(results.len());
        }

        // "High roll": all dices > 3
        if results.iter().all(|&d| d > 3) {
            return Combination::HighRoll(results);
        }

        // "Low roll": all dices < 3
        if results.iter().all(|&d| d < 3) {
            return Combination::LowRoll(results);
        }

        // Default to "Any" combination
        Combination::Any(results)
    }

    pub fn score(&self) -> u32 {
        match self {
            Combination::FourTwoOne(len) => 10 * (3.0 / *len as f32) as u32,
            Combination::Strike(dice) if *dice == 1 => 7,
            Combination::Ace(dice) | Combination::Strike(dice) => (*dice).into(),
            Combination::Straight(_, _) => 4,
            Combination::HighRoll(_) => 3,
            Combination::LowRoll(_) => 2,
            Combination::Any(_) => 1,
        }
    }

    unsafe fn discriminant(&self) -> u8 {
        *std::ptr::from_ref::<Self>(self).cast::<u8>()
    }
}

impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Combination {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ord = self.score().cmp(&other.score());

        if ord == Ordering::Equal {
            ord = unsafe { self.discriminant().cmp(&other.discriminant()) };
        }

        if ord == Ordering::Equal {
            ord = match (self, other) {
                (Combination::Any(a), Combination::Any(b)) => {
                    a.iter().sum::<DiceResult>().cmp(&b.iter().sum())
                }
                (Combination::Strike(dice), Combination::Strike(other_dice)) => {
                    dice.cmp(other_dice)
                }
                _ => ord,
            };
        }

        ord
    }
}

impl fmt::Display for Combination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}  [{}¢]",
            match self {
                Combination::FourTwoOne(_) => "Four-Two-One!".to_string(),
                Combination::Ace(dice) => format!("{dice} Ace"),
                Combination::Strike(dice) => format!("{dice} Strike"),
                Combination::Straight(dice, len) => format!("Straight {}", {
                    (0..*len)
                        .rev()
                        .map(|i| (dice - i as u8).to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                }),
                Combination::HighRoll(dices) => format!(
                    "High Roll {}",
                    dices.iter().map(ToString::to_string).collect::<String>()
                ),
                Combination::LowRoll(dices) => format!(
                    "Low Roll {}",
                    dices.iter().map(ToString::to_string).collect::<String>()
                ),
                Combination::Any(dices) => {
                    dices.iter().map(ToString::to_string).collect::<String>()
                }
            },
            self.score()
        )
    }
}
