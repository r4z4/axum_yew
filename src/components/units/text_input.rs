use yew::prelude::*;
use crate::log;
use web_sys::{HtmlInputElement, EventTarget};
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub placeholder: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange: Callback<String> = props.handle_onchange.clone();
    let result: UseStateHandle<String> = use_state(|| "".to_owned());
    let input_string: UseStateHandle<String> = use_state(|| "".to_owned());
    let onchange = {
        let input_string_cloned: UseStateHandle<String> = input_string.clone();
        Callback::from(
            move |event: Event| {
                let target: EventTarget = event.target().unwrap();
                let input: HtmlInputElement = target.unchecked_into::<HtmlInputElement>();
                let value = input.value();
                handle_onchange.emit(value);
            }
        )
    };
    html! {
        <input type="text" name={props.name.clone()} placeholder={props.placeholder.clone()} onchange={onchange}/>
    }
}