use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;

use crate::components::units::register_form::{RegisterForm, Data};
use crate::router::Route;

#[function_component(Register)]
pub fn register() -> Html {
    let register_form_submit = Callback::from(|data: Data| {
        log!("Username is", data.username);
        log!("Passwrod is", data.password);
    });
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div>
            <div id={"register-form"}>
                <RegisterForm form_title={"Register"} onsubmit={register_form_submit} />
                <div class="sub-form">
                    <Link<Route> to={Route::Login}>{"Already have an account? Login Here"}</Link<Route>>
                </div>
            </div>
        </div>
    }
}