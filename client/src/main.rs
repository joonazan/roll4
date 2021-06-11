#![recursion_limit = "256"]
mod dicecomponent;
use dicecomponent::*;
use aper::{StateMachineContainerProgram};
use aper_yew::{ClientBuilder, View, ViewContext};
use state::dice::{Dice, DiceTransition};
use yew::prelude::*;

#[derive(Clone)]
struct DiceView;

type DiceProgram = StateMachineContainerProgram<Dice>;

impl View for DiceView {
    type Callback = DiceTransition;
    type State = DiceProgram;

    fn view(&self, state: &Self::State, context: &ViewContext<Self::Callback>) -> Html {
        let roll_buttons = (1..=6).map(move |n| html!{
            <button onclick=context.callback.reform(move |_| Some(DiceTransition::Roll(n)))>{n}</button>
        });
        html! {<>
            <div>
                {"Roll: "}{for roll_buttons}
            </div>
            <DiceComponent timestamp=context.time rolls=state.0.rolls.clone() last_rolled=state.0.last_rolled.clone()
               reroll_cb=context.callback.clone().reform(|x: Option<Vec<bool>>| x.map(DiceTransition::Reroll)) />
        </>}
    }
}

fn main() {
    ClientBuilder::new(DiceView).mount_to_body();
}
