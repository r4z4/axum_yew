use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{"Yew Heading"}</h1>
            <div>{"Yew Div"}</div>
            <div>
                <ul>
                    <li>{"First"}</li>
                    <li>{"Second"}</li>
                    <li>{"Third"}</li>
                </ul>
            </div>
        </>
    }
}