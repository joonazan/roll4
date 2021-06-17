use aper::StateMachine;
use state::Character;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlObjectElement, SvgElement};
use yew::prelude::*;

pub struct CharacterSheet {
    svg_doc: NodeRef,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub character: Character,
    pub cb: Callback<<Character as StateMachine>::Transition>,
}

pub enum Message {
    SvgLoaded,
}
use Message::*;

impl Component for CharacterSheet {
    type Message = Message;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            svg_doc: NodeRef::default(),
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SvgLoaded => self.update_svg(),
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.update_svg();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let character = &self.props.character;
        html! { <div class="charactersheet">
        <object ref=self.svg_doc.clone() onload=self.link.callback(|_| SvgLoaded)
                type="image/svg+xml" data="client/sheet.svg" id="svg"></object>
        <input type="text" class="name" value=character.name.value()/>
        <input type="text" class="habitat" value=character.habitat.value()/>
        <input type="text" class="note_1"/>
        <input type="text" class="note_2"/>
        <input type="text" class="note_3"/>
        <input type="text" class="note_4"/>
        <input type="text" class="note_5"/>
        <input type="text" class="note_6"/>
        <input type="text" class="note_7"/>
        <input type="text" class="note_8"/>
        <input type="text" class="effect_value_1"/>
        <input type="text" class="effect_value_2"/>
        <input type="text" class="effect_value_3"/>
        <input type="text" class="effect_value_4"/>
        <input type="text" class="effect_value_5"/>
        <input type="text" class="effect_value_6"/>
        <input type="text" class="effect_value_7"/>
        <input type="text" class="effect_value_8"/>
        <input type="text" class="effect_value_9"/>
        <input type="text" class="effect_value_10"/>
        <input type="text" class="effect_value_11"/>
        <input type="text" class="effect_value_12"/>
        <input type="text" class="effect_value_13"/>
        <input type="text" class="effect_value_14"/>
        <input type="text" class="effect_value_15"/>
        <input type="text" class="effect_value_16"/>
        <input type="text" class="effect_value_17"/>
        <input type="text" class="effect_value_18"/>
        <input type="text" class="effect_name_1"/>
        <input type="text" class="effect_name_2"/>
        <input type="text" class="effect_name_3"/>
        <input type="text" class="effect_name_4"/>
        <input type="text" class="effect_name_5"/>
        <input type="text" class="effect_name_6"/>
        <input type="text" class="effect_name_7"/>
        <input type="text" class="effect_name_8"/>
        <input type="text" class="effect_name_9"/>
        <input type="text" class="effect_name_10"/>
        <input type="text" class="effect_name_11"/>
        <input type="text" class="effect_name_12"/>
        <input type="text" class="effect_name_13"/>
        <input type="text" class="effect_name_14"/>
        <input type="text" class="effect_name_15"/>
        <input type="text" class="effect_name_16"/>
        <input type="text" class="effect_name_17"/>
        <input type="text" class="effect_name_18"/>
        </div> }
    }
}

impl CharacterSheet {
    fn update_svg(&self) {
        if let Some(doc) = self
            .svg_doc
            .cast::<HtmlObjectElement>()
            .unwrap()
            .content_document()
        {
            let set_highlight = |el: Element, v| {
                el.unchecked_into::<SvgElement>()
                    .style()
                    .set_property("visibility", if v { "visible" } else { "hidden" })
                    .unwrap();
            };

            let set_influ = |i, v| {
                set_highlight(
                    doc.get_element_by_id(&format!("influence_{}", i)).unwrap(),
                    v,
                );
            };

            let character = &self.props.character;

            let ip = *character.influence_points.value();
            for i in 1..=ip {
                set_influ(i, true);
            }
            for i in ip + 1..=9 {
                set_influ(i, false);
            }

            let set_mp = |i, v| {
                set_highlight(
                    doc.get_element_by_id(&format!("memory_{}", i)).unwrap(),
                    v,
                );
            };
            let mp = *character.memory_points.value();
            for i in 1..=mp {
                set_mp(i, true);
            }
            for i in mp + 1..=9 {
                set_mp(i, false);
            }

            for i in -5..=5 {
                set_highlight(doc.get_element_by_id(&format!("gravity_{}", i)).unwrap(), false);
            }

            let body_descriptions = ["wounded", "beaten", "ok"];
            let body = *character.body.value();
            for (i, desc) in body_descriptions.iter().enumerate() {
                set_highlight(doc.get_element_by_id(&format!("body_{}", desc)).unwrap(), i+1 == body as usize);
            }

            let mind_descriptions = ["shaken", "stressed", "ok"];
            let mind = *character.mind.value();
            for (i, desc) in mind_descriptions.iter().enumerate() {
                set_highlight(doc.get_element_by_id(&format!("mind_{}", desc)).unwrap(), i+1 == mind as usize);
            }
        }
    }
}
