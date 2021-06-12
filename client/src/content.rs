use crate::dicecomponent::DiceComponent;
use aper::data_structures::ListItem;
use state::Character;
use state::{Game, GameTransition};
use uuid::Uuid;
use yew::prelude::*;

pub struct Content {
    state: Game,
    character: Option<Uuid>,
    link: ComponentLink<Self>,
    cb: Callback<Option<GameTransition>>,
}

pub enum ContentMsg {
    SelectCharacter(Uuid),
}
use ContentMsg::*;

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
            character: None,
            link,
            cb: props.cb,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SelectCharacter(id) => self.character = Some(id),
        }
        true
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
        let reroll = self.character.map(|me| {
            self.cb
                .clone()
                .reform(move |x: Option<Vec<bool>>| x.map(move |x| GameTransition::Reroll(x, me)))
        });

        let characters = self.state.characters.iter().map(|ListItem{value, id, ..}| {
            html! { <div>
                {format!("{:?}", value)}
                <button onclick=self.link.callback(move |_| SelectCharacter(id))>{"This is me!"}</button>
            </div> }
        });

        let add_char = GameTransition::CharacterTransition(
            self.state.characters.append(Character::default()).1,
        );
        html! {<>
            <div>
                {"Roll: "}{for roll_buttons}
            </div>
            <DiceComponent roll_id=dice.roll_id rolls=dice.rolls.clone() last_rolled=dice.last_rolled.clone()
             reroll_cb=reroll />

            {for characters}
            <button onclick=self.cb.reform(move |_| Some(add_char.clone()))>{"Add Character"}</button>
        </>}
    }
}
