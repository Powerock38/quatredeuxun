use combination::*;
use dice::*;
use rand::prelude::*;

mod combination;
mod dice;

fn main() {
    let mut rng = thread_rng();

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
