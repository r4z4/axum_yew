use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub color: String,
}

#[function_component(Nav)]
pub fn nav(props: &Props) -> Html {
    html! {
        <div>
            <ul id={"nav-list"}>
                <li><Link<Route> to={Route::Provider}>{"Providers"}</Link<Route>></li>
                <li><Link<Route> to={Route::Patient}>{"Patients"}</Link<Route>></li>
                <li><Link<Route> to={Route::Insurer}>{"Insurers"}</Link<Route>></li>
                <li><Link<Route> to={Route::Iro}>{"Iros"}</Link<Route>></li>
                <li><Link<Route> to={Route::EligibleCase}>{"Eligible Cases"}</Link<Route>></li>
                <li><Link<Route> to={Route::Home}>{"🏠"}</Link<Route>></li>
            </ul>
        </div>
    }
}
