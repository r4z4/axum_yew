use crate::components::units::button::Button;
use crate::components::units::date_input::DateInput;
use crate::components::units::radio::Radio;
use crate::components::units::text_input::TextInput;
use gloo::console::log;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct Data {
    pub denial_reason: String,
    pub expedited: String,
    pub date_forwarded: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form_title: String,
    pub onsubmit: Callback<Data>,
}

#[function_component(EligibleCaseForm)]
pub fn eligible_case_form(props: &Props) -> Html {
    let state: UseStateHandle<Data> = use_state(|| Data::default());

    let cloned_state: UseStateHandle<Data> = state.clone();
    let denial_reason_changed: Callback<String> = Callback::from(move |denial_reason| {
        let mut data: Data = cloned_state.deref().clone();
        data.denial_reason = denial_reason;
        cloned_state.set(data);
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let date_forwarded_changed: Callback<String> = Callback::from(move |date_forwarded| {
        let mut data: Data = cloned_state.deref().clone();
        data.date_forwarded = date_forwarded;
        cloned_state.set(data);
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let expedited_changed: Callback<String> = Callback::from(move |expedited| {
        let mut data: Data = cloned_state.deref().clone();
        data.expedited = expedited;
        cloned_state.set(data);
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit: Callback<SubmitEvent> = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });
    html! {
        <div>
            <h3>{props.form_title.deref().clone()}</h3>
            <form onsubmit={onsubmit}>
                <TextInput name="denial_reason" placeholder="Denial Reason" handle_onchange={denial_reason_changed} />
                <DateInput name="date_forwarded" handle_onchange={date_forwarded_changed} />
                <Radio name={"expedited"} handle_onchange={expedited_changed} />
                <Button label="Submit" />
            </form>
        </div>
    }
}
