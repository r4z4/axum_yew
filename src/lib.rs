use yew::prelude::*;
use gloo::console::log;
use serde::Serialize;

#[derive(Serialize)]
struct CurrentUser {
    username: String,
    role: String,
}

#[function_component(App)]
pub fn app() -> Html {
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

    html! {
        <>
            <h1 class="main_title">{"Yew Main Heading"}</h1>
            <h1 class={title_class}>{"Yew Alt Heading"}</h1>
            <div>{"Yew Div"}</div>
            <div>
                <ul>
                    <li>{"First"}</li>
                    <li>{"Second"}</li>
                    <li>{"Third"}</li>
                    <li>{"Fourth"}</li>
                </ul>
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