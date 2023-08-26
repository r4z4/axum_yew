use yew::prelude::*;
use crate::log;
use web_sys::{HtmlInputElement, EventTarget};
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(Radio)]
pub fn radio(props: &Props) -> Html {
    let handle_onchange: Callback<String> = props.handle_onchange.clone();
    let input_bool: UseStateHandle<String> = use_state(|| "no".to_owned());
    let onchange = {
        let input_bool_cloned: UseStateHandle<String> = input_bool.clone();
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
        <fieldset>
        <legend>{"Expedited"}</legend>
        <div>
            <input type="radio" id={"yes"} name={props.name.clone()} value={"yes"} />
            <label for="yes">{"Yes"}</label>
        </div>

        <div>
            <input type="radio" id={"no"} name={props.name.clone()} value={"no"} checked={true} />
            <label for="no">{"No"}</label>
        </div>
        </fieldset>
    }
}