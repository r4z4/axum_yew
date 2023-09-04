use crate::components::units::button::Button;
use crate::components::units::email_input::EmailInput;
use crate::components::units::tel_input::TelInput;
use crate::components::units::text_input::TextInput;
use gloo::console::log;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct Data {
    pub name: String,

    pub f_name: String,
    pub l_name: String,

    pub address_1: String,
    pub address_2: String,
    pub contact_f_name: String,
    pub contact_l_name: String,
    // pub deleted_at: Date,
    pub phone: String,
    pub email: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub form_title: String,
    pub onsubmit: Callback<Data>,
}

#[function_component(SimpleForm)]
pub fn simple_form(props: &Props) -> Html {
    let state: UseStateHandle<Data> = use_state(|| Data::default());

    // singular name field used for entites (provider, iro etc..)

    let cloned_state: UseStateHandle<Data> = state.clone();
    let name_changed: Callback<String> = Callback::from(move |name| {
        let mut data: Data = cloned_state.deref().clone();
        data.name = name;
        cloned_state.set(data);
    });

    // l_name & f_name fields used for persons (patient, user)

    let cloned_state: UseStateHandle<Data> = state.clone();
    let f_name_changed: Callback<String> = Callback::from(move |f_name| {
        let mut data: Data = cloned_state.deref().clone();
        data.f_name = f_name;
        cloned_state.set(data);
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let l_name_changed: Callback<String> = Callback::from(move |l_name| {
        let mut data: Data = cloned_state.deref().clone();
        data.l_name = l_name;
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

    let cloned_state: UseStateHandle<Data> = state.clone();
    let contact_f_name_changed: Callback<String> = Callback::from(move |contact_f_name| {
        let mut data: Data = cloned_state.deref().clone();
        data.contact_f_name = contact_f_name;
        cloned_state.set(data);
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let contact_l_name_changed: Callback<String> = Callback::from(move |contact_l_name| {
        let mut data: Data = cloned_state.deref().clone();
        data.contact_l_name = contact_l_name;
        cloned_state.set(data);
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let email_changed: Callback<String> = Callback::from(move |email| {
        let mut data: Data = cloned_state.deref().clone();
        data.email = email;
        cloned_state.set(data);
    });

    let cloned_state: UseStateHandle<Data> = state.clone();
    let phone_changed: Callback<String> = Callback::from(move |phone| {
        let mut data: Data = cloned_state.deref().clone();
        data.phone = phone;
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
        <div class={"simple-form"}>
        <h3>{props.form_title.deref().clone()}</h3>
            <form onsubmit={onsubmit}>
                <TextInput name="name" placeholder="Name" handle_onchange={name_changed} />

                <TextInput name="f_name" placeholder="First Name" handle_onchange={f_name_changed} />
                <TextInput name="l_name" placeholder="Last Name" handle_onchange={l_name_changed} />

                <TextInput name="address_1" placeholder="Address" handle_onchange={addr_1_changed} />
                <TextInput name="address_2" placeholder="Apt/Ste" handle_onchange={addr_2_changed} />
                <TextInput name="contact_f_name" placeholder="Contact First Name" handle_onchange={contact_f_name_changed} />
                <TextInput name="contact_l_name" placeholder="Contact Last Nname" handle_onchange={contact_l_name_changed} />

                <TelInput name="phone" placeholder="Phone" handle_onchange={phone_changed} />
                <EmailInput name="email" placeholder="Email" handle_onchange={email_changed} />
                <Button label="Submit" />
            </form>
        </div>
    }
}
