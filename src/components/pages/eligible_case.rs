use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(EligibleCase)]
pub fn eligible_case() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Home);
    });
    html! {
        <div>
            <h1>{"Eligible Case"}</h1>
            <button onclick={onclick}>{"Go Home"}</button>
        </div>
    }
}