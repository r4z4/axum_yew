use yew::prelude::*;
use crate::components::units::text_input::TextInput;
use crate::components::units::button::Button;
use wasm_bindgen::JsCast;
use gloo::console::log;
use std::ops::Deref;

#[derive(Default, Clone)]
pub struct Data {
    pub name: String,
    pub address_1: String,
    pub address_2: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(SimpleForm)]
pub fn simple_form(props: &Props) -> Html {
    let state: UseStateHandle<Data> = use_state(|| Data::default());
    
    let cloned_state: UseStateHandle<Data> = state.clone();
    let name_changed: Callback<String> = Callback::from(move |name| {
        let mut data: Data = cloned_state.deref().clone();
        data.name = name;
        cloned_state.set(data);    
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let addr_1_changed: Callback<String> = Callback::from(move |addr_1| {
        let mut data: Data = cloned_state.deref().clone();
        data.address_1 = addr_1;
        cloned_state.set(data);    
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let addr_2_changed: Callback<String> = Callback::from(move |addr_2| {
        let mut data: Data = cloned_state.deref().clone();
        data.address_2 = addr_2;
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
            <TextInput name="name" placeholder="Name" handle_onchange={name_changed} />
            <TextInput name="address_1" placeholder="Address" handle_onchange={addr_1_changed} />
            <TextInput name="address_2" placeholder="Apt/Ste" handle_onchange={addr_2_changed} />
            <Button label="Submit" />
        </form>
    }
}