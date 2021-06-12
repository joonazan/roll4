#![recursion_limit = "256"]
mod content;
mod dicecomponent;
use aper::StateMachineContainerProgram;
use aper_yew::{ClientBuilder, View, ViewContext};
use content::Content;
use state::{Game, GameTransition};
use yew::prelude::*;

#[derive(Clone)]
struct GameView;

impl View for GameView {
    type Callback = GameTransition;
    type State = StateMachineContainerProgram<Game>;

    fn view(&self, state: &Self::State, context: &ViewContext<Self::Callback>) -> Html {
        html! {
            <Content state=state.0.clone() cb=context.callback.clone() />
        }
    }
}

fn main() {
    ClientBuilder::new(GameView).mount_to_body();
}
