use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::units::insurer_display::{Entity, InsurerDisplay};
use crate::components::units::simple_form::{Data, SimpleForm};
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
            <details>
                <summary>{"Add an Insurer"}</summary>
                <div class={"form_container"}>
                    <SimpleForm form_title={"Add Insurer"} onsubmit={insurer_form_submit} />
                </div>
            </details>
            <button onclick={onclick}>{"Go Home"}</button>
            <div id={"insurer_display"}>
                <InsurerDisplay title={"⚕️ Insurer Data 🥼"} entity={Entity::Insurer} on_load={data_display_loaded} />
            </div>
        </div>
    }
}
