use yew::prelude::*;
use gloo::console::log;
use yew_router::prelude::*;
use crate::components::units::eligible_case_form::{EligibleCaseForm, Data};

use crate::router::Route;

#[function_component(Patient)]
pub fn patient() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Home);
    });
    let eligible_case_form_submit = Callback::from(|data: Data| {
        log!("Denial Reason is", data.denial_reason);
        log!("Expedited is", data.expedited);
    });
    html! {
        <div>
            <h1>{"Patient"}</h1>
            <EligibleCaseForm onsubmit={eligible_case_form_submit} />
            <button onclick={onclick}>{"Go Home"}</button>
        </div>
    }
}