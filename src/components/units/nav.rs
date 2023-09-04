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
            <h1>{"Nav"}</h1>
            <ul id={"nav-list"}>
                <li><Link<Route> to={Route::Provider}>{"Provider"}</Link<Route>></li>
                <li><Link<Route> to={Route::Patient}>{"Patient"}</Link<Route>></li>
                <li><Link<Route> to={Route::Insurer}>{"Insurer"}</Link<Route>></li>
                <li><Link<Route> to={Route::Iro}>{"Iro"}</Link<Route>></li>
                <li><Link<Route> to={Route::EligibleCase}>{"Eligible Case"}</Link<Route>></li>
            </ul>
        </div>
    }
}
