#![recursion_limit = "256"]
mod dicecomponent;
use aper::data_structures::ListItem;
use aper::StateMachineContainerProgram;
use aper_yew::{ClientBuilder, View, ViewContext};
use dicecomponent::*;
use state::{character::Character, Game, GameTransition};
use yew::prelude::*;

#[derive(Clone)]
struct GameView;

impl View for GameView {
    type Callback = GameTransition;
    type State = StateMachineContainerProgram<Game>;

    fn view(&self, state: &Self::State, context: &ViewContext<Self::Callback>) -> Html {
        let roll_buttons = (1..=6).map(move |n| html!{
            <button onclick=context.callback.reform(move |_| Some(GameTransition::Roll(n)))>{n}</button>
        });
        let dice = &state.0.dice;

        // TODO allow selecting your character
        // requires making a Component to hold extra state
        let me = state.0.characters.iter().next().map(|x| x.id);
        let reroll = if let Some(me) = me {
            Some(
                context
                    .callback
                    .clone()
                    .reform(move |x: Option<Vec<bool>>| {
                        x.map(move |x| GameTransition::Reroll(x, me))
                    }),
            )
        } else {
            None
        };

        let add_char =
            GameTransition::CharacterTransition(state.0.characters.append(Character::default()).1);
        html! {<>
            <div>
                {"Roll: "}{for roll_buttons}
            </div>
            <DiceComponent roll_id=dice.roll_id rolls=dice.rolls.clone() last_rolled=dice.last_rolled.clone()
             reroll_cb=reroll />

            <button onclick=context.callback.reform(move |_| Some(add_char.clone()))>{"Add Character"}</button>
            {for state.0.characters.iter().map(|ListItem{value, ..}| format!("{:?}", value))}
        </>}
    }
}

fn main() {
    ClientBuilder::new(GameView).mount_to_body();
}
