use gloo::console::log;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub entity: Entity,
    pub on_load: Callback<String>,
}

#[derive(PartialEq)]
pub enum Entity {
    Provider,
    Patient,
    Insurer,
    Iro,
    EligibleCase,
}

fn vec_to_html(list: &Vec<Patient>) -> Vec<Html> {
    list.iter()
        .map(|patient| {
            html! {<ul class="data-display">
                <li>{patient.patient_f_name.clone()}</li>
                <li>{patient.patient_l_name.clone()}</li>
                <li>{patient.patient_zip.clone()}</li>
            </ul>}
        })
        .collect()
}

impl Entity {
    pub fn to_string(&self) -> String {
        match self {
            Entity::Provider => "Provider Entity".to_owned(),
            Entity::Patient => "Patient".to_owned(),
            Entity::Insurer => "insurer".to_owned(),
            Entity::Iro => "iro".to_owned(),
            Entity::EligibleCase => "eligible_case".to_owned(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Patient {
    pub patient_id: i32,
    pub patient_f_name: String,
    pub patient_l_name: String,
    pub patient_email: Option<String>,
    pub patient_dob: Option<String>,
    pub patient_zip: Option<String>,
    pub patient_address_1: Option<String>,
    pub patient_address_2: Option<String>,
    pub deleted_at: Option<String>,
    pub created_by: Option<i32>,
}

#[styled_component(PatientDisplay)]
pub fn patient_display(props: &Props) -> Html {
    let entity = use_state(|| "patient".to_owned());
    let data: UseStateHandle<Option<Vec<Patient>>> = use_state(|| None);
    let cloned_data = data.clone();
    let onclick = {
        let entity = entity.clone();
        Callback::from(move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:3000/get_patients")
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<Vec<Patient>>()
                    .await
                    .unwrap();

                // log!(serde_json::to_string_pretty(&response).unwrap());
                data.set(Some(response))
            });
        })
    };
    props.on_load.emit("Data Display Loaded".to_string());
    html! {
        <div class={"data-display"}>
            <h1>{&props.title}</h1>
            <p>{&props.entity.to_string()}</p>
            if cloned_data.is_some() {
                {vec_to_html(cloned_data.as_ref().unwrap())}
            }
            <button {onclick}>{"Get Data"}</button>
        </div>
    }
}
