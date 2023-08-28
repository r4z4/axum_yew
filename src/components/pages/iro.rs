use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;

use crate::components::units::simple_form::{SimpleForm, Data};
use crate::components::units::data_display::{DataDisplay, Entity};
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
            <div id={"iro_form"}>
                <SimpleForm form_title={"Add IRO"} onsubmit={iro_form_submit} />
                <button onclick={onclick}>{"Go Home"}</button>
            </div>
            <div id={"iro_display"}>
                <DataDisplay title={"💊 Iro Data 💉"} entity={Entity::Iro} on_load={data_display_loaded} />
            </div>
        </div>
    }
}