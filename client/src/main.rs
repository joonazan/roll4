use aper::{StateMachineContainerProgram, Timestamp};
use aper_yew::{ClientBuilder, View, ViewContext};
use state::dice::{Dice, DiceTransition};
use std::iter::repeat;
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
            <DiceComponent timestamp=context.time rolls=state.0.rolls.clone() last_rolled=state.0.last_rolled.clone() />
            <div>
                {"Roll: "}{for roll_buttons}
            </div>
        </>}
    }
}

fn main() {
    ClientBuilder::new(DiceView).mount_to_body();
}

struct DiceComponent {
    last_update: Timestamp,
    dice: Vec<(u8, bool)>,
}

#[derive(Properties, Clone)]
struct DiceProps {
    timestamp: Timestamp,
    rolls: Vec<u8>,
    last_rolled: Vec<bool>,
}

impl Component for DiceComponent {
    type Message = ();
    type Properties = DiceProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            last_update: props.timestamp,
            dice: props.rolls.into_iter().zip(repeat(false)).collect(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.last_update != props.timestamp {
            self.last_update = props.timestamp;
            self.dice = props
                .rolls
                .into_iter()
                .zip(
                    self.dice
                        .iter()
                        .map(|(_, c)| c)
                        .zip(props.last_rolled.into_iter())
                        .map(|(&c, rolled)| if rolled { !c } else { c })
                        .chain(repeat(false)),
                )
                .collect();
            true
        } else {
            false
        }
    }

    fn update(&mut self, _: ()) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { <div>
            {for self.dice.iter().map(|(x, c)|
                html!{<span class=if *c {"die dieani1"} else {"die dieani2"}>{x}</span>}
            )}
        </div> }
    }
}
