#![recursion_limit = "256"]

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
            <DiceComponent timestamp=context.time rolls=state.0.rolls.clone() last_rolled=state.0.last_rolled.clone() cb=context.callback.clone() />
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
    dice: Vec<Die>,
    selected: Vec<bool>,
    link: ComponentLink<Self>,
    cb: Callback<Option<DiceTransition>>,
}

struct Die {
    roll: u8,
    class: bool,
}

#[derive(Properties, Clone)]
struct DiceProps {
    timestamp: Timestamp,
    rolls: Vec<u8>,
    last_rolled: Vec<bool>,
    cb: Callback<Option<DiceTransition>>,
}

enum DiceMsg {
    Select(usize),
}
use DiceMsg::*;

impl Component for DiceComponent {
    type Message = DiceMsg;
    type Properties = DiceProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let dice: Vec<Die> = props
            .rolls
            .into_iter()
            .map(|r| Die {
                roll: r,
                class: false,
            })
            .collect();
        let selected = vec![false; dice.len()];
        Self {
            last_update: props.timestamp,
            dice,
            selected,
            link,
            cb: props.cb,
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
                        .map(|Die { class, .. }| class)
                        .zip(props.last_rolled.into_iter())
                        .map(|(&c, rolled)| if rolled { !c } else { c })
                        .chain(repeat(false)),
                )
                .map(|(r, c)| Die { roll: r, class: c })
                .collect();
            self.selected = vec![false; self.dice.len()];
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Select(n) => {
                self.selected[n] = !self.selected[n];
                true
            }
        }
    }

    fn view(&self) -> Html {
        let cannot_reroll = self.selected.iter().all(|&s| s == false);

        html! { <>
            <div class="diebox">
            {for self.dice.iter().enumerate().map(|(i, d)|
                html!{<span data-selected=self.selected[i]
                      class=if d.class {"die dieani1"} else {"die dieani2"}
                      onclick=self.link.callback(move |_| Select(i))>
                      {d.roll}
                </span>}
            )}
            </div>
            <button disabled=cannot_reroll
             onclick=self.cb.reform({
                 let mask: Option<Vec<bool>> = if cannot_reroll {
                     None
                 }else {
                     Some(self.selected.clone())
                 };
                 move |_| mask.clone().map(DiceTransition::Reroll)
             })>
                {"Reroll"}
            </button>
        </>}
    }
}
