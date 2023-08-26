use yew::prelude::*;
use crate::components::units::text_input::TextInput;
use crate::components::units::radio::Radio;
use crate::components::units::button::Button;
use wasm_bindgen::JsCast;
use gloo::console::log;
use std::ops::Deref;

#[derive(Default, Clone)]
pub struct Data {
    pub denial_reason: String,
    pub expedited: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
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
        <form onsubmit={onsubmit}>
            <TextInput name="denial_reason" placeholder="Denial Reason" handle_onchange={denial_reason_changed} />
            <Radio name={"expedited"} handle_onchange={expedited_changed} />
            <Button label="Submit" />
        </form>
    }
}