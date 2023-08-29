use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;

use crate::components::units::simple_form::{SimpleForm, Data};
use crate::components::units::provider_display::{ProviderDisplay, Entity};
use crate::router::Route;

#[function_component(Provider)]
pub fn provider() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Home);
    });
    let provider_form_submit = Callback::from(|data: Data| {
        log!("Name is", data.name);
        log!("Addr 1 is", data.address_1);
        log!("Addr 2 is", data.address_2);
    });
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div>
            <h1>{"Provider"}</h1>
            <details>
                <summary>{"Add a Provider"}</summary>
                <div class={"form_container"}>
                    <SimpleForm form_title={"Add Provider"} onsubmit={provider_form_submit} />
                </div>
            </details>
            <button onclick={onclick}>{"Go Home"}</button>
            <div id={"provider_display"}>
                <ProviderDisplay title={"👨‍⚕️ Provider Data 👩‍⚕️"} entity={Entity::Provider} on_load={data_display_loaded} />
            </div>
        </div>
    }
}