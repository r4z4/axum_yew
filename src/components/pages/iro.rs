use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::units::iro_display::{Entity, IroDisplay};
use crate::components::units::simple_form::{Data, SimpleForm};
use crate::router::Route;

#[function_component(Iro)]
pub fn iro() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Home);
    });
    let iro_form_submit = Callback::from(|data: Data| {
        log!("Name is", data.name);
        log!("Addr 1 is", data.address_1);
        log!("Addr 2 is", data.address_2);
    });
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div>
            <h1>{"Iro"}</h1>
            <details>
                <summary>{"Add an IRO"}</summary>
                <div class={"form_container"}>
                    <SimpleForm form_title={"Add IRO"} onsubmit={iro_form_submit} />
                </div>
            </details>
            <button onclick={onclick}>{"Go Home"}</button>
            <div id={"iro_display"}>
                <IroDisplay title={"💊 Iro Data 💉"} entity={Entity::Iro} on_load={data_display_loaded} />
            </div>
        </div>
    }
}
