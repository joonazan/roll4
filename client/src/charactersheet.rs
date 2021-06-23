use aper::data_structures::ListItem;
use aper::StateMachine;
use state::Character;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlObjectElement, SvgElement};
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
    InfluenceClicked(u8),
    MemoryClicked(u8),
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
            SvgLoaded => {
                self.init_svg();
                self.update_svg();
            }
            InfluenceClicked(x) => {
                self.props
                    .cb
                    .emit(self.props.character.map_influence_points(|ip| {
                        ip.replace(if x > *ip.value() { x } else { x - 1 })
                    }));
            }
            MemoryClicked(x) => {
                self.props.cb.emit(
                    self.props.character.map_memory_points(|mp| {
                        mp.replace(if x > *mp.value() { x } else { x - 1 })
                    }),
                );
            }
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
        <input type="text" class="name" value=character.name.value()
                 onchange=self.props.cb.reform({
                     let character = character.clone();
                     move |i: ChangeData| match i {
                         ChangeData::Value(v) => character.map_name(|n| n.replace(v)),
                         _ => unreachable!()
                     }
                 })/>
        <input type="text" class="habitat" value=character.habitat.value()
                 onchange=self.props.cb.reform({
                     let character = character.clone();
                     move |i: ChangeData| match i {
                         ChangeData::Value(v) => character.map_habitat(|n| n.replace(v)),
                         _ => unreachable!()
                     }
                 })/>
        {for character.notes.iter().enumerate().map(|(i, ListItem{id, value, ..})|{
            html!{
                <input type="text" class=format!("note_{}", i) value=value.value()
                    onchange=self.props.cb.reform({
                        let character = character.clone();
                        move |i: ChangeData| match i {
                            ChangeData::Value(v) => character.map_notes(move |n| n.map_item(id, |i| i.replace(v))),
                            _ => unreachable!()
                        }
                    })/>
            }
        })}
        {for (0..18).map(|i| {
            html!{<>
                <input type="text" class=format!("effect_value_{}", i)/>
                <input type="text" class=format!("effect_name_{}", i)/>
            </>}
        })}
        </div> }
    }
}

impl CharacterSheet {
    fn init_svg(&self) {
        let doc = self
            .svg_doc
            .cast::<HtmlObjectElement>()
            .unwrap()
            .content_document()
            .unwrap();

        // Inject stylesheet
        let style = doc
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "style")
            .unwrap();
        style.set_text_content(Some("@import url(svg.css)"));
        doc.first_child().unwrap().append_child(&style).unwrap();

        // Add click handlers to clickable parts of the svg
        for i in 1..=9 {
            {
                let link = self.link.clone();
                let c = Closure::wrap(
                    Box::new(move || link.send_message(InfluenceClicked(i))) as Box<dyn Fn()>
                );
                get_influence(&doc, i)
                    .unchecked_into::<SvgElement>()
                    .set_onclick(Some(c.as_ref().unchecked_ref()));

                // TODO don't do this, it leaks memory when characters are deleted
                c.forget();
            }
            {
                let link = self.link.clone();
                let c = Closure::wrap(
                    Box::new(move || link.send_message(MemoryClicked(i))) as Box<dyn Fn()>
                );
                get_memory(&doc, i)
                    .unchecked_into::<SvgElement>()
                    .set_onclick(Some(c.as_ref().unchecked_ref()));

                // TODO don't do this, it leaks memory when characters are deleted
                c.forget();
            }
        }

        for i in 1..=3 {
            {
                let character = self.props.character.clone();
                let cb = self.props.cb.clone();
                let c =
                    Closure::wrap(
                        Box::new(move || cb.emit(character.map_body(|b| b.replace(i))))
                            as Box<dyn Fn()>,
                    );
                get_body(&doc, i)
                    .unchecked_into::<SvgElement>()
                    .set_onclick(Some(c.as_ref().unchecked_ref()));

                // TODO don't do this, it leaks memory when characters are deleted
                c.forget();
            }
            {
                let character = self.props.character.clone();
                let cb = self.props.cb.clone();
                let c =
                    Closure::wrap(
                        Box::new(move || cb.emit(character.map_mind(|b| b.replace(i))))
                            as Box<dyn Fn()>,
                    );
                get_mind(&doc, i)
                    .unchecked_into::<SvgElement>()
                    .set_onclick(Some(c.as_ref().unchecked_ref()));

                // TODO don't do this, it leaks memory when characters are deleted
                c.forget();
            }
        }
    }

    fn update_svg(&self) {
        if let Some(doc) = self
            .svg_doc
            .cast::<HtmlObjectElement>()
            .unwrap()
            .content_document()
        {
            let set_highlight = |el: SvgElement, v| {
                el.set_attribute("data-on", if v { "true" } else { "false" })
                    .unwrap();
            };

            let set_influ = |i, v| {
                set_highlight(get_influence(&doc, i), v);
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
                set_highlight(get_memory(&doc, i), v);
            };
            let mp = *character.memory_points.value();
            for i in 1..=mp {
                set_mp(i, true);
            }
            for i in mp + 1..=9 {
                set_mp(i, false);
            }

            for i in -5..=5 {
                set_highlight(get_gravity(&doc, i), false);
            }

            let body = *character.body.value();
            for i in 1..=3 {
                set_highlight(get_body(&doc, i), i == body);
            }

            let mind = *character.mind.value();
            for i in 1..=3 {
                set_highlight(get_mind(&doc, i), i == mind);
            }
        }
    }
}

static BODY_DESCRIPTIONS: &[&str] = &["wounded", "beaten", "ok"];
static MIND_DESCRIPTIONS: &[&str] = &["shaken", "stressed", "ok"];

fn get_influence(doc: &Document, i: u8) -> SvgElement {
    doc.get_element_by_id(&format!("influence_{}", i))
        .unwrap()
        .unchecked_into::<SvgElement>()
}

fn get_memory(doc: &Document, i: u8) -> SvgElement {
    doc.get_element_by_id(&format!("memory_{}", i))
        .unwrap()
        .unchecked_into::<SvgElement>()
}

fn get_body(doc: &Document, i: u8) -> SvgElement {
    doc.get_element_by_id(&format!("body_{}", BODY_DESCRIPTIONS[(i - 1) as usize]))
        .unwrap()
        .unchecked_into::<SvgElement>()
}

fn get_mind(doc: &Document, i: u8) -> SvgElement {
    doc.get_element_by_id(&format!("mind_{}", MIND_DESCRIPTIONS[(i - 1) as usize]))
        .unwrap()
        .unchecked_into::<SvgElement>()
}

fn get_gravity(doc: &Document, i: i8) -> SvgElement {
    doc.get_element_by_id(&format!("gravity_{}", i))
        .unwrap()
        .unchecked_into::<SvgElement>()
}
