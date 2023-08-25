use yew::prelude::*;
use crate::components::units::text_input::TextInput;
use crate::components::units::button::Button;
use wasm_bindgen::JsCast;
use gloo::console::log;

#[function_component(SimpleForm)]
pub fn simple_form() -> Html {
    let username_state: UseStateHandle<String> = use_state(|| "No Username Set".to_owned());
    let button_count_state: UseStateHandle<u32> = use_state(|| 0_u32);
    let cloned_username_state = username_state.clone();
    let username_changed: Callback<String> = Callback::from(move |username| {
            cloned_username_state.set(username);
    });
    let cloned_button_count_state = button_count_state.clone();
    let button_clicked = Callback::from(move |_| {
        let count: u32 = *cloned_button_count_state;
        cloned_button_count_state.set(count + 1);
    });
    html! {
        <div>
            <TextInput name="username" placeholder="Username" handle_onchange={username_changed} />
            <Button label="Submit" onclick={button_clicked} />
            <p>{"Username: "}{&*username_state}</p>
            <p>{"Button has been clicked "}{*button_count_state}{" times"}</p>
        </div>
    }
}