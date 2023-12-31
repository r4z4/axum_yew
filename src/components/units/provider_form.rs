use crate::components::units::button::Button;
use crate::components::units::text_input::TextInput;
use gloo::console::log;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct Data {
    pub provider_name: String,
    pub provider_address_1: String,
    pub provider_address_2: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form_title: String,
    pub onsubmit: Callback<Data>,
}

#[function_component(ProviderForm)]
pub fn provider_form(props: &Props) -> Html {
    let state: UseStateHandle<Data> = use_state(|| Data::default());

    let cloned_state: UseStateHandle<Data> = state.clone();
    let name_changed: Callback<String> = Callback::from(move |name| {
        let mut data: Data = cloned_state.deref().clone();
        data.provider_name = name;
        cloned_state.set(data);
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let addr_1_changed: Callback<String> = Callback::from(move |addr_1| {
        let mut data: Data = cloned_state.deref().clone();
        data.provider_address_1 = addr_1;
        cloned_state.set(data);
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let addr_2_changed: Callback<String> = Callback::from(move |addr_2| {
        let mut data: Data = cloned_state.deref().clone();
        data.provider_address_2 = addr_2;
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
                <TextInput name="provider_name" placeholder="Provider Name" handle_onchange={name_changed} />
                <TextInput name="provider_address_1" placeholder="Address" handle_onchange={addr_1_changed} />
                <TextInput name="provider_address_2" placeholder="Apt/Ste" handle_onchange={addr_2_changed} />
                <Button label="Submit" />
            </form>
        </div>
    }
}
