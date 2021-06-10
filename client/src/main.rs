use yew::prelude::*;
use aper_yew::{View, ViewContext, ClientBuilder};
use aper::{StateMachineContainerProgram};

use state::dice::{Dice, DiceTransition};

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
        return html! {<>
            <div>
                {for state.0.rolls.iter().map(|x| html!{<span class="die">{x}</span>})}
            </div>
            <div>
                {"Roll: "}{for roll_buttons}
            </div>
        </>}
    }
}

fn main() {
    ClientBuilder::new(DiceView).mount_to_body();
}
