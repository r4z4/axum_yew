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

fn vec_to_html(list: &Vec<Provider>) -> Vec<Html> {
    list.iter()
        .map(|provider| {
            html! {<ul class="data-display">
                <li>{provider.provider_name.clone()}</li>
                <li>{provider.provider_phone.clone()}</li>
                <li>{provider.provider_zip.clone()}</li>
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
pub struct Provider {
    pub provider_id: i32,
    pub provider_name: String,
    pub provider_phone: Option<String>,
    pub provider_zip: Option<String>,
    pub provider_address_1: Option<String>,
    pub provider_address_2: Option<String>,
    pub provider_contact_f_name: Option<String>,
    pub provider_contact_l_name: Option<String>,
    pub deleted_at: Option<String>,
    pub created_by: Option<i32>,
}

#[styled_component(ProviderDisplay)]
pub fn provider_display(props: &Props) -> Html {
    let entity = use_state(|| "provider".to_owned());
    let data: UseStateHandle<Option<Vec<Provider>>> = use_state(|| None);
    let cloned_data = data.clone();
    let onclick = {
        let entity = entity.clone();
        Callback::from(move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:3000/get_providers")
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<Vec<Provider>>()
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
            <h4>{format!("Click Below to Fetch {} Data.", &props.entity.to_string())}</h4>
            if cloned_data.is_some() {
                {vec_to_html(cloned_data.as_ref().unwrap())}
            }
            <button {onclick}>{"Get Data"}</button>
        </div>
    }
}
