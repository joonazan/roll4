use aper::data_structures::List;
use js_sys::{Array, Date};
use state::Character;
use wasm_bindgen::JsValue;
use web_sys::{Blob, Url};
use yew::prelude::*;

pub struct SaveButton {
    save: Option<Save>,
    characters: List<Character>,
    link: ComponentLink<Self>,
}

struct Save {
    name: String,
    object_url: String,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: List<Character>,
}

pub struct GenerateSave;

impl Component for SaveButton {
    type Message = GenerateSave;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            save: None,
            characters: props.state,
            link,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        if let Some(Save { object_url, .. }) = &self.save {
            Url::revoke_object_url(object_url).unwrap();
        }

        let arr = Array::new();
        arr.set(
            0,
            JsValue::from_str(&serde_json::to_string(&self.characters).unwrap()),
        );
        let blob = Blob::new_with_str_sequence(&arr).unwrap();

        let date = Date::new_0();
        let day: String = date.to_date_string().into();
        let name = format!("{} {}:{}", day, date.get_hours(), date.get_minutes());

        self.save = Some(Save {
            name,
            object_url: Url::create_object_url_with_blob(&blob).unwrap(),
        });
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.characters = props.state;
        false
    }

    fn view(&self) -> Html {
        html! { <div id="savewidget">
            <button onclick=self.link.callback(|_| GenerateSave)>{"Generate Save"}</button>
            <br/>
            {if let Some(Save{name, object_url}) = &self.save {
                let href: yew::html::Href = object_url.clone().into();
                html!{<a download=name.to_string()+".json" href=href>{format!("Download {}", name)}</a>}
            } else {
                html!{}
            }}
        </div> }
    }
}
