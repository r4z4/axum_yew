use crate::components::pages::eligible_case::EligibleCase;
use crate::components::pages::home::Home;
use crate::components::pages::insurer::Insurer;
use crate::components::pages::iro::Iro;
use crate::components::pages::login::Login;
use crate::components::pages::patient::Patient;
use crate::components::pages::provider::Provider;
use crate::components::pages::register::Register;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/provider")]
    Provider,
    #[at("/patient")]
    Patient,
    #[at("/insurer")]
    Insurer,
    #[at("/iro")]
    Iro,
    #[at("/eligible_case")]
    EligibleCase,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Provider => html! { <Provider /> },
        Route::Patient => html! { <Patient /> },
        Route::Insurer => html! { <Insurer /> },
        Route::Iro => html! { <Iro /> },
        Route::EligibleCase => html! { <EligibleCase /> },
    }
}
