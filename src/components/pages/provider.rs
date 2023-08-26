use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Provider)]
pub fn provider() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Home);
    });
    html! {
        <div>
            <h1>{"Provider"}</h1>
            <button onclick={onclick}>{"Go Home"}</button>
        </div>
    }
}