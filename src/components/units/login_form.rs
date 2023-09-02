use yew::prelude::*;
use yewdux::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use serde_json::json;
use reqwasm::http::Request;
use gloo::console::log;
use std::ops::Deref;
use yewdux::functional::use_store;
use yewdux::store::*;

use crate::components::units::text_input::TextInput;
use crate::components::units::button::Button;
use crate::store::auth_store::AuthStore;

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
    let store = use_store::<PersistentStore<AuthStore>>;
    let state: UseStateHandle<Data> = use_state(|| Data::default());
    
    let username_changed: Callback<Event> = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let username: String = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.username = username;   
        });

    let password_changed: Callback<Event> = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let password: String = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.password = password;   
        });

    // let form_onsubmit = real_login_form_submit.clone();
    let cloned_state = state.clone();

    let onsubmit: Callback<SubmitEvent> = {
        let dispatch = store.dispatch().clone();
        store
            .dispatch()
            .reduce_callback_with(move |state, event: FocusEvent| {
                event.prevent_default();
                let username = state.username.clone();
                let password = state.password.clone();
                let dispatch = dispatch.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = login_user(username, password).await;
                    let token = response.token;
                    // Need this to be a move because moving this token into this closure
                    dispatch.reduce(move  |state| state.token = token);  
                })
            });
    };

    let token = if let Some(state) = store.state() {
        state.token.clone()
    } else {
        String::new() // Just get new empty string
    };
    
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