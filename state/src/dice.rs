use aper::{StateMachine, Transition};
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dice {
    pub rolls: Vec<u8>,
    pub last_rolled: Vec<bool>,
    rng: ChaCha12Rng,
}

impl Default for Dice {
    fn default() -> Self {
        Self {
            rolls: vec![],
            last_rolled: vec![],
            rng: ChaCha12Rng::from_entropy(),
        }
    }
}

#[derive(Transition, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DiceTransition {
    Roll(u8),
    Reroll(Vec<bool>),
}
use DiceTransition::*;

impl StateMachine for Dice {
    type Transition = DiceTransition;
    fn apply(&mut self, t: Self::Transition) {
        let d4 = Uniform::from(1..=4);
        match t {
            Roll(x) => {
                self.rolls = (0..x).map(|_| d4.sample(&mut self.rng)).collect();
                self.last_rolled = vec![true; x as usize];
            }
            Reroll(mask) => {
                let rng = &mut self.rng;
                self.rolls = self
                    .rolls
                    .iter()
                    .zip(&mask)
                    .map(|(r, m)| if *m { d4.sample(rng) } else { *r })
                    .collect();
                self.last_rolled = mask;
            }
        }
    }
}
