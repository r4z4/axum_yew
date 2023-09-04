use crate::log;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub placeholder: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(EmailInput)]
pub fn email_input(props: &Props) -> Html {
    let handle_onchange: Callback<String> = props.handle_onchange.clone();
    let input_string: UseStateHandle<String> = use_state(|| "".to_owned());
    let onchange = {
        let input_string_cloned: UseStateHandle<String> = input_string.clone();
        Callback::from(move |event: Event| {
            let target: EventTarget = event.target().unwrap();
            let input: HtmlInputElement = target.unchecked_into::<HtmlInputElement>();
            let value = input.value();
            handle_onchange.emit(value);
        })
    };
    html! {
        <input type="email" name={props.name.clone()} placeholder={props.placeholder.clone()} onchange={onchange}/>
    }
}
