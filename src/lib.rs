use yew::prelude::*;
use gloo::console::log;
use serde::Serialize;
use stylist::{yew::styled_component, style};
use stylist::Style;
use yew_router::prelude::*;

mod components;
mod router;

use components::units::main_title::{MainTitle, Color};
use components::units::simple_form::{SimpleForm, Data};
use crate::components::units::nav::Nav;
use crate::router::{switch, Route};

const CSS_FILE: &str = include_str!("main.css");

#[derive(Serialize)]
struct CurrentUser {
    username: Option<String>,
    role: Option<String>,
}

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(CSS_FILE).unwrap();
    let username = "Jim_01";
    let current_user = CurrentUser {
        username: Some(username.to_owned()),
        role: Some("admin".to_owned()),
    };
    log!("The username is", username);
    log!(serde_json::to_string_pretty(&current_user).unwrap());
    let title_class = "alt_title";
    let p_class = "main_p";
    let message: Option<&str> = None;

    let roles: Vec<&str> = vec!["admin", "patient", "gov", "office"];

    let main_title_loaded = Callback::from(|message: String| log!(message));

    let custom_form_submit = Callback::from(|data: Data| {
        log!("Name is", data.name);
        log!("Addr 1 is", data.address_1);
        log!("Addr 2 is", data.address_2);
    });

    html! {
        <div class={stylesheet}>   
            <MainTitle title="🏥 External Review Portal for {INSERT STATE HERE} 🩺" color={Color::Okay} on_load={main_title_loaded} />
            if current_user.username.is_some() {
                <BrowserRouter>
                    // Nav needs to be child of BrowserRouter
                    <Nav color={"black"} />
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            }
            <ul>
                <li>{"🩺 Doc"}</li>
            </ul>
            <SimpleForm form_title={"Lib Form"} onsubmit={custom_form_submit} />
            if p_class == "main_p" {
                <p>{"This is the main p"}</p>
            } else {
                <p>{"This is the alt p"}</p>
            }

            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"No user messages."}</p>
            }

            <ul>
                {vec_to_html(roles)}
            </ul>
        </div>
    }
}

fn vec_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html!{<li>{item}</li>}).collect()
}