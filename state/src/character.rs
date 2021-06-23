use aper::{data_structures::{Atom, List}, StateMachine};
use serde::{Deserialize, Serialize};

#[derive(StateMachine, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Character {
    pub name: Atom<String>,
    pub habitat: Atom<String>,
    pub body: Atom<u8>,
    pub mind: Atom<u8>,
    pub memory_points: Atom<u8>,
    pub influence_points: Atom<u8>,
    pub preferred_gravity: Atom<u8>,
    pub notes: List<Atom<String>>,
}

impl Default for Character {
    fn default() -> Self {
        let mut notes = List::new();
        for _ in 0..8 {
            notes.apply(notes.append(Atom::new("".to_string())).1);
        }

        Self {
            name: Atom::new("".to_string()),
            habitat: Atom::new("Brighttown".to_string()),
            body: Atom::new(3),
            mind: Atom::new(3),
            memory_points: Atom::new(0),
            influence_points: Atom::new(1),
            preferred_gravity: Atom::new(5),
            notes,
        }
    }
}
