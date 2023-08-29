use yew::prelude::*;
use gloo::console::log;
use stylist::{yew::styled_component, style};
use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use std::ops::Deref;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub entity: Entity,
    pub on_load: Callback<String>
}

#[derive(PartialEq)]
pub enum Entity {
    Provider,
    Patient,
    Insurer,
    Iro,
    EligibleCase,
}

fn vec_to_html(list: &Vec<Insurer>) -> Vec<Html> {
    list.iter().map(|insurer| html!
        {<ul class="data-display">
            <li>{insurer.insurer_name.clone()}</li>
            <li>{insurer.insurer_phone.clone()}</li>
            <li>{insurer.insurer_zip.clone()}</li>
        </ul>}
    ).collect()
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
pub struct Insurer {
    pub insurer_id: i32,
    pub insurer_name: String,
    pub insurer_phone: Option<String>,
    pub insurer_zip: Option<String>,
    pub insurer_address_1: Option<String>,
    pub insurer_address_2: Option<String>,
    pub insurer_contact_f_name: Option<String>,
    pub insurer_contact_l_name: Option<String>,
    pub deleted_at: Option<String>,
    pub created_by: Option<i32>,
}

#[styled_component(InsurerDisplay)]
pub fn insurer_display(props: &Props) -> Html {
    let entity = use_state(|| "insurer".to_owned());
    let data: UseStateHandle<Option<Vec<Insurer>>> = use_state(|| None);
    let cloned_data = data.clone();
    let onclick = {
        let entity = entity.clone();
        Callback::from(move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:3000/get_insurers")
                //.header("x-auth-token", &state.token)
                .send()
                .await
                // FIXME unwrap_or_else - handle
                .unwrap()
                .json::<Vec<Insurer>>()
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