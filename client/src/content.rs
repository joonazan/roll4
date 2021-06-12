use crate::dicecomponent::DiceComponent;
use aper::data_structures::ListItem;
use state::Character;
use state::{Game, GameTransition};
use yew::prelude::*;

pub struct Content {
    state: Game,
    link: ComponentLink<Self>,
    cb: Callback<Option<GameTransition>>,
}

pub enum ContentMsg {}

#[derive(Properties, Clone)]
pub struct ContentProps {
    pub state: Game,
    pub cb: Callback<Option<GameTransition>>,
}

impl Component for Content {
    type Message = ContentMsg;

    type Properties = ContentProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: props.state,
            link,
            cb: props.cb,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        (if self.state != props.state {
            self.state = props.state;
            true
        } else {
            false
        } || {
            if self.cb != props.cb {
                self.cb = props.cb;
                true
            } else {
                false
            }
        })
    }

    fn view(&self) -> Html {
        let roll_buttons = (1..=6).map(move |n| {
            html! {
                <button onclick=self.cb.reform(move |_| Some(GameTransition::Roll(n)))>{n}</button>
            }
        });
        let dice = &self.state.dice;

        // TODO allow selecting your character
        // requires making a Component to hold extra state
        let me = self.state.characters.iter().next().map(|x| x.id);
        let reroll =
            if let Some(me) = me {
                Some(self.cb.clone().reform(move |x: Option<Vec<bool>>| {
                    x.map(move |x| GameTransition::Reroll(x, me))
                }))
            } else {
                None
            };

        let add_char = GameTransition::CharacterTransition(
            self.state.characters.append(Character::default()).1,
        );
        html! {<>
            <div>
                {"Roll: "}{for roll_buttons}
            </div>
            <DiceComponent roll_id=dice.roll_id rolls=dice.rolls.clone() last_rolled=dice.last_rolled.clone()
             reroll_cb=reroll />

            <button onclick=self.cb.reform(move |_| Some(add_char.clone()))>{"Add Character"}</button>
            {for self.state.characters.iter().map(|ListItem{value, ..}| format!("{:?}", value))}
        </>}
    }
}
