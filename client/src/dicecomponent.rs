use std::iter::repeat;
use std::num::Wrapping;
use yew::prelude::*;

pub struct DiceComponent {
    last_update: Wrapping<u8>,
    dice: Vec<Die>,
    selected: Vec<bool>,
    link: ComponentLink<Self>,
    reroll_cb: Option<Callback<Option<Vec<bool>>>>,
}

struct Die {
    roll: u8,
    class: bool,
}

#[derive(Properties, Clone)]
pub struct DiceProps {
    pub roll_id: Wrapping<u8>,
    pub rolls: Vec<u8>,
    pub last_rolled: Vec<bool>,
    pub reroll_cb: Option<Callback<Option<Vec<bool>>>>,
}

pub enum DiceMsg {
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
            last_update: props.roll_id,
            dice,
            selected,
            link,
            reroll_cb: props.reroll_cb,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let a = if self.last_update != props.roll_id {
            self.last_update = props.roll_id;
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
        };

        let b = if self.reroll_cb != props.reroll_cb {
            self.reroll_cb = props.reroll_cb;
            true
        } else {
            false
        };

        a || b
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
        let cannot_reroll = self.reroll_cb == None || self.selected.iter().all(|&s| s == false);
        let cb = if cannot_reroll {
            Callback::noop()
        } else {
            let mask = self.selected.clone();
            self.reroll_cb
                .clone()
                .unwrap()
                .reform(move |_| Some(mask.clone()))
        };

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
            <button disabled=cannot_reroll onclick=cb>{"Reroll"}</button>
        </>}
    }
}
