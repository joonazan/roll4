use aper::{data_structures::Atom, StateMachine};
use serde::{Deserialize, Serialize};

#[derive(StateMachine, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Character {
    body: Atom<u8>,
    mind: Atom<u8>,
    memory_points: Atom<u8>,
    influence_points: Atom<u8>,
    preferred_gravity: Atom<u8>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            body: Atom::new(3),
            mind: Atom::new(3),
            memory_points: Atom::new(0),
            influence_points: Atom::new(3),
            preferred_gravity: Atom::new(5),
        }
    }
}
