pub mod character;
pub mod dice;

use aper::data_structures::List;
use aper::{StateMachine, Transition};
pub use character::Character;
use dice::Dice;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Game {
    pub dice: Dice,
    pub characters: List<Character>,
}

#[derive(Transition, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameTransition {
    Roll(u8),
    Reroll(Vec<bool>, Uuid),
    CharacterTransition(<List<Character> as StateMachine>::Transition),
    Load(List<Character>),
}
use GameTransition::*;

impl StateMachine for Game {
    type Transition = GameTransition;

    fn apply(&mut self, transition: Self::Transition) {
        match transition {
            Roll(x) => self.dice.apply(dice::DiceTransition::Roll(x)),
            Reroll(mask, character) => {
                let dice = &mut self.dice;
                self.characters
                    .apply(self.characters.map_item(character, |c| {
                        c.map_influence_points(|i| {
                            let old = *i.value();
                            if old > 0 {
                                dice.apply(dice::DiceTransition::Reroll(mask));
                                i.replace(old - 1)
                            } else {
                                i.replace(old)
                            }
                        })
                    }));
            }
            CharacterTransition(t) => self.characters.apply(t),
            Load(x) => self.characters = x,
        }
    }
}
