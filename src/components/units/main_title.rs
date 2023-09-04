use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
    pub on_load: Callback<String>,
}

#[derive(PartialEq)]
pub enum Color {
    Base,
    Okay,
    Error,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Base => "base".to_owned(),
            Color::Okay => "okay".to_owned(),
            Color::Error => "error".to_owned(),
        }
    }
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    props.on_load.emit("Main Title Loaded".to_string());
    html! {
        <div>
            <h1 class={props.color.to_string()}>{&props.title}</h1>
        </div>
    }
}
