use stylist::css;
use yew::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextAreaInput)]
pub fn text_area_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });
    html! {
        <>
        <pre>
        <textarea class={css!("white-space: pre;")}rows="10" cols="80" type="text" name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()}>
        </textarea>
        </pre>
        </>
    }
}
