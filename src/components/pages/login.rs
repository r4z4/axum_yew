use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;

use crate::components::units::data_display::{DataDisplay, Entity};
use crate::components::units::login_form::{LoginForm, Data};
use crate::router::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Login);
    });
    let login_form_submit = Callback::from(|data: Data| {
        log!("Username is", data.username);
        log!("Passwrod is", data.password);
    });
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div>
            <h1>{"Insurer"}</h1>
            <div id={"login_form"}>
                <LoginForm form_title={"Login"} onsubmit={login_form_submit} />
                <button onclick={onclick}>{"Go Home"}</button>
            </div>
        </div>
    }
}