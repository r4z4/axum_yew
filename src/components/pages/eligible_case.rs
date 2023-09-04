use crate::components::units::eligible_case_display::{EligibleCaseDisplay, Entity};
use crate::components::units::eligible_case_form::{Data, EligibleCaseForm};
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(EligibleCase)]
pub fn eligible_case() -> Html {
    let eligible_case_form_submit = Callback::from(|data: Data| {
        log!("Denial Reason is", data.denial_reason);
        log!("Expedited is", data.expedited);
    });
    let data_display_loaded = Callback::from(|message: String| log!(message));
    html! {
        <div  class={"entity-page"}>
            <h1>{"Eligible Cases"}</h1>
            <details>
            <summary>{"Add an Insurer"}</summary>
                <div class={"form_container"}>
                    <EligibleCaseForm form_title={"Add Eligible Case"} onsubmit={eligible_case_form_submit} />
                </div>
            </details>
            <div id={"insurer_display"}>
                <EligibleCaseDisplay title={"⚕️ Eligible Case Data 🥼"} entity={Entity::EligibleCase} on_load={data_display_loaded} />
            </div>
        </div>
    }
}
