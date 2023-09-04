use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::units::provider_display::{Entity, ProviderDisplay};
use crate::components::units::simple_form::{Data, SimpleForm};
use crate::router::Route;

#[function_component(Provider)]
pub fn provider() -> Html {
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
            <div id={"provider_display"}>
                <ProviderDisplay title={"👨‍⚕️ Provider Data 👩‍⚕️"} entity={Entity::Provider} on_load={data_display_loaded} />
            </div>
        </div>
    }
}
