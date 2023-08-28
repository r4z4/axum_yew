use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;

use crate::components::units::simple_form::{SimpleForm, Data};
use crate::components::units::data_display::{DataDisplay, Entity};
use crate::router::Route;

#[function_component(Insurer)]
pub fn insurer() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Home);
    });
    let insurer_form_submit = Callback::from(|data: Data| {
        log!("Name is", data.name);
        log!("Addr 1 is", data.address_1);
        log!("Addr 2 is", data.address_2);
    });
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div>
            <h1>{"Insurer"}</h1>
            <div id={"insurer_form"}>
                <SimpleForm form_title={"Add Insurer"} onsubmit={insurer_form_submit} />
                <button onclick={onclick}>{"Go Home"}</button>
            </div>
            <div id={"insurer_display"}>
                <DataDisplay title={"⚕️ Insurer Data 🥼"} entity={Entity::Insurer} on_load={data_display_loaded} />
            </div>
        </div>
    }
}