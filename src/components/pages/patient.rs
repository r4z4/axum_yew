use yew::prelude::*;
use gloo::console::log;
use yew_router::prelude::*;
use crate::components::units::simple_form::{SimpleForm, Data};

use crate::router::Route;

#[function_component(Patient)]
pub fn patient() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Home);
    });
    let patient_form_submit = Callback::from(|data: Data| {
        log!("Name is", data.name);
        log!("Addr 1 is", data.address_1);
        log!("Addr 2 is", data.address_2);
    });
    html! {
        <div>
            <h1>{"Patient"}</h1>
            <SimpleForm onsubmit={patient_form_submit} />
            <button onclick={onclick}>{"Go Home"}</button>
        </div>
    }
}