use aper::{StateMachine, Transition};
use serde::{Deserialize, Serialize};
use rand::SeedableRng;
use rand::distributions::{Distribution, Uniform};
use rand_chacha::ChaCha12Rng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dice {
    pub rolls: Vec<u8>,
    pub last_rolled: Vec<bool>,
    rng: ChaCha12Rng
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
    Roll(u8)
}
use DiceTransition::*;

impl StateMachine for Dice {
    type Transition = DiceTransition;
    fn apply(&mut self, t: Self::Transition) {
        match t {
            Roll(x) => {
                let d4 = Uniform::from(1..=4);
                self.rolls = (0..x).map(|_| d4.sample(&mut self.rng)).collect();
                self.last_rolled = vec![true; x as usize];
            }
        }
    }
}
