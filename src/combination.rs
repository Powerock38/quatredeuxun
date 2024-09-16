use core::fmt;
use std::cmp::Ordering;

use crate::dice::NB_DICES;

pub type DiceResult = u8;

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum Combination {
    Nenette = 0,
    Any(DiceResult, DiceResult, DiceResult),
    Serie(DiceResult), // highest dice in the serie
    Strike(DiceResult),
    Ace(DiceResult), // can't be 1
    QuatreCentVingtEtUn,
}

impl Combination {
    pub fn get(results: &mut [DiceResult; NB_DICES]) -> Self {
        results.sort_unstable();
        let [low_dice, mid_dice, high_dice] = *results;

        match (high_dice, mid_dice, low_dice) {
            (4, 2, 1) => Combination::QuatreCentVingtEtUn,
            (2, 2, 1) => Combination::Nenette,
            (a, b, c) if a == b && b == c => Combination::Strike(high_dice),
            (x, 1, 1) => Combination::Ace(x),
            (h, m, l) if l == m - 1 && m == h - 1 => Combination::Serie(h),
            _ => Combination::Any(high_dice, mid_dice, low_dice),
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Combination::QuatreCentVingtEtUn => 8,
            Combination::Ace(dice) => *dice,
            Combination::Strike(dice) if *dice == 1 => 7,
            Combination::Strike(dice) => *dice,
            Combination::Serie(_) => 2,
            Combination::Any(_, _, _) => 1,
            Combination::Nenette => 0,
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
        let mut ord = unsafe { self.discriminant().cmp(&other.discriminant()) };

        if ord == Ordering::Equal {
            ord = self.value().cmp(&other.value());
        }

        if ord == Ordering::Equal {
            ord = match (self, other) {
                (Combination::Any(a1, b1, c1), Combination::Any(a2, b2, c2)) => {
                    (a1 + b1 + c1).cmp(&(a2 + b2 + c2))
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
        match self {
            Combination::QuatreCentVingtEtUn => write!(f, "421 !!!"),
            Combination::Ace(dice) => write!(f, "{dice} purs"),
            Combination::Strike(dice) => write!(f, "Brelan de {dice}"),
            Combination::Serie(dice) => write!(f, "Suite {} {} {}", dice, dice - 1, dice - 2),
            Combination::Any(a, b, c) => write!(f, "{a}{b}{c}"),
            Combination::Nenette => write!(f, "Nenette !"),
        }
    }
}
