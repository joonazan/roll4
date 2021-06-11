use yew::prelude::*;
use std::iter::repeat;
use aper::Timestamp;

pub struct DiceComponent {
    last_update: Timestamp,
    dice: Vec<Die>,
    selected: Vec<bool>,
    link: ComponentLink<Self>,
    reroll_cb: Callback<Option<Vec<bool>>>,
}

struct Die {
    roll: u8,
    class: bool,
}

#[derive(Properties, Clone)]
pub struct DiceProps {
    pub timestamp: Timestamp,
    pub rolls: Vec<u8>,
    pub last_rolled: Vec<bool>,
    pub reroll_cb: Callback<Option<Vec<bool>>>,
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
            last_update: props.timestamp,
            dice,
            selected,
            link,
            reroll_cb: props.reroll_cb,
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
             onclick=self.reroll_cb.reform({
                 let mask: Option<Vec<bool>> = if cannot_reroll {
                     None
                 }else {
                     Some(self.selected.clone())
                 };
                 move |_| mask.clone()
             })>
                {"Reroll"}
            </button>
        </>}
    }
}
