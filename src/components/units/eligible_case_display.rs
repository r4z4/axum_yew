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

fn vec_to_html(list: &Vec<EligibleCase>) -> Vec<Html> {
    list.iter()
        .map(|eligible_case| {
            html! {<ul class="data-display">
                <li>{eligible_case.eligible_case_id.clone()}</li>
                <li>{eligible_case.patient_id.clone()}</li>
                <li>{eligible_case.denial_reason.clone()}</li>
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
pub struct EligibleCase {
    pub eligible_case_id: i32,
    pub patient_id: i32,
    pub insurer_id: i32,
    pub provider_id: Option<i32>,
    pub iro_id: Option<i32>,
    pub denial_reason: String,
    pub expedited: Option<String>,
    pub date_forwarded: Option<String>,
    pub eligibility_notice: Option<String>,
    pub eligible_correspondence: Option<String>,
    pub insurer_notified: Option<String>,
    pub decision_date: Option<String>,
    pub iro_decision: Option<String>,
    pub file_closed: Option<String>,
    pub invoice_amount: Option<f64>,
    pub deleted_at: Option<String>,
}

#[styled_component(EligibleCaseDisplay)]
pub fn eligible_case_display(props: &Props) -> Html {
    let entity = use_state(|| "eligible_case".to_owned());
    let data: UseStateHandle<Option<Vec<EligibleCase>>> = use_state(|| None);
    let cloned_data = data.clone();
    let onclick = {
        let entity = entity.clone();
        Callback::from(move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:3000/get_eligible_cases")
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<Vec<EligibleCase>>()
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
