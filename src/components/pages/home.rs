use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>
            <ul>
            <li><Link<Route> to={Route::Provider}>{"Provider"}</Link<Route>></li>
            <li><Link<Route> to={Route::Patient}>{"Patient"}</Link<Route>></li>
            <li><Link<Route> to={Route::Insurer}>{"Insurer"}</Link<Route>></li>
            <li><Link<Route> to={Route::Iro}>{"Iro"}</Link<Route>></li>
            <li><Link<Route> to={Route::EligibleCase}>{"Eligible Case"}</Link<Route>></li>
            </ul>
        </div>
    }
}