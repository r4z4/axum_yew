use yew::prelude::*;
use gloo::console::log;
use serde::Serialize;
use stylist::{yew::styled_component, style};
use stylist::Style;

mod components;

use components::units::main_title::{MainTitle, Color};
use components::units::simple_form::SimpleForm;

const CSS_FILE: &str = include_str!("main.css");

#[derive(Serialize)]
struct CurrentUser {
    username: String,
    role: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(CSS_FILE).unwrap();
    let username = "Jim_01";
    let user = CurrentUser {
        username: username.to_owned(),
        role: "admin".to_owned(),
    };
    log!("The username is", username);
    log!(serde_json::to_string_pretty(&user).unwrap());
    let title_class = "alt_title";
    let p_class = "main_p";
    let message: Option<&str> = None;

    let roles: Vec<&str> = vec!["admin", "patient", "gov", "office"];

    let main_title_loaded = Callback::from(|message: String| log!(message));

    html! {
        <>
            <h1 class="main_title">{"Yew Main Heading"}</h1>
            <h1 class={title_class}>{"Yew Alt Heading"}</h1>
            <div>{"Yew Div"}</div>
            <div class={stylesheet}>
            <MainTitle title="Component Title In Div :)" color={Color::Okay} on_load={main_title_loaded} />
                <ul>
                    <li>{"First"}</li>
                    <li>{"Second"}</li>
                    <li>{"Third"}</li>
                    <li>{"Fourth"}</li>
                </ul>
                <SimpleForm />
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
        </>
    }
}

fn vec_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html!{<li>{item}</li>}).collect()
}