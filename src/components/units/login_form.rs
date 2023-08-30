use yew::prelude::*;
use crate::components::units::text_input::TextInput;
use crate::components::units::button::Button;
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsCast;
use serde_json::json;
use reqwasm::http::Request;
use gloo::console::log;
use std::ops::Deref;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub password: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form_title: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiLoginResponse {
    pub id: i32,
    pub username: String,
    pub token: String,
}

pub async fn login_user(username: String, password: String) -> ApiLoginResponse {
    let body = json!({
        "username": username,
        "password": password
    });
    let response = Request::post("http://localhost:3000/users/login")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiLoginResponse>()
        .await
        .unwrap();

    response
}

// const real_login_form_submit = Callback::from(|event: SubmitEvent| {
//     event.prevent_default();
//     let username = state.username.clone();
//     let password = state.password.clone();
//     wasm_bindgen_futures::spawn_local(async move {
//         let response = login_user(username, password).await;
//         log!(response.token)
//     })
// });

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
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


    // let form_onsubmit = real_login_form_submit.clone();
    let cloned_state = state.clone();
    let onsubmit: Callback<SubmitEvent> = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let username = state.username.clone();
        let password = state.password.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = login_user(username, password).await;
            // Use this
            log!(response.token)
        })
    });
    html! {
        <div>
            <h3>{props.form_title.deref().clone()}</h3>
            <form onsubmit={onsubmit}>
                <TextInput name="username" placeholder="Userame" handle_onchange={username_changed} />
                <TextInput name="password" placeholder="Passwrod" handle_onchange={password_changed} />
                <Button label="Login" />
            </form>
        </div>
    }
}