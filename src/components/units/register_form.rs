use yew::prelude::*;
use crate::components::units::text_input::TextInput;
use crate::components::units::email_input::EmailInput;
use crate::components::units::button::Button;
use wasm_bindgen::JsCast;
use gloo::console::log;
use std::ops::Deref;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub password: String,
    pub re_password: String,
    pub email: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form_title: String,
    pub onsubmit: Callback<Data>,
}

#[function_component(RegisterForm)]
pub fn register_form(props: &Props) -> Html {
    let state: UseStateHandle<Data> = use_state(|| Data::default());
    
    let cloned_state: UseStateHandle<Data> = state.clone();
    let username_changed: Callback<String> = Callback::from(move |username| {
        let mut data: Data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);    
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let password_changed: Callback<String> = Callback::from(move |password| {
        let mut data: Data = cloned_state.deref().clone();
        data.password = password;
        cloned_state.set(data);    
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let re_password_changed: Callback<String> = Callback::from(move |re_password| {
        let mut data: Data = cloned_state.deref().clone();
        data.re_password = re_password;
        cloned_state.set(data);    
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let email_changed: Callback<String> = Callback::from(move |password| {
        let mut data: Data = cloned_state.deref().clone();
        data.password = password;
        cloned_state.set(data);    
    });


    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit: Callback<SubmitEvent> = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });
    html! {
        <div>
            <h3>{props.form_title.deref().clone()}</h3>
            <form onsubmit={onsubmit}>
                <TextInput name="username" placeholder="Userame" handle_onchange={username_changed} />
                <TextInput name="password" placeholder="Passwrod" handle_onchange={password_changed} />
                <TextInput name="re_password" placeholder="Reenter Passwrod" handle_onchange={re_password_changed} />
                <EmailInput name="email" placeholder="Email" handle_onchange={email_changed} />
                <Button label="Submit" />
            </form>
        </div>
    }
}