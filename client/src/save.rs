use aper::data_structures::List;
use gloo_file::{
    callbacks::{read_as_bytes, FileReader},
    FileList,
};
use js_sys::{Array, Date};
use state::Character;
use wasm_bindgen::prelude::*;
use web_sys::{Blob, HtmlInputElement, Url};
use yew::prelude::*;

pub struct SaveButton {
    save: Option<Save>,
    _file_read: Option<FileReader>,
    fileselect: NodeRef,
    props: Props,
    link: ComponentLink<Self>,
}

struct Save {
    name: String,
    object_url: String,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub characters: List<Character>,
    pub load: Callback<List<Character>>,
}

pub enum Msg {
    GenerateSave,
    Load,
}
use Msg::*;

impl Component for SaveButton {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            save: None,
            _file_read: None,
            fileselect: Default::default(),
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            GenerateSave => {
                if let Some(Save { object_url, .. }) = &self.save {
                    Url::revoke_object_url(object_url).unwrap();
                }

                let arr = Array::new();
                arr.set(
                    0,
                    JsValue::from_str(&serde_json::to_string(&self.props.characters).unwrap()),
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
            Load => {
                let filelist: FileList = self
                    .fileselect
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .files()
                    .unwrap()
                    .into();
                let load = self.props.load.clone();
                self._file_read = Some(read_as_bytes(&filelist[0], move |res| {
                    load.emit(serde_json::from_slice(&res.unwrap()).unwrap());
                }));

                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
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

            {"Load: "}<input type="file" ref=self.fileselect.clone() onchange=self.link.callback(|_| Load)/>
        </div> }
    }
}
