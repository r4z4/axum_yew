use yew::prelude::*;
use crate::components::units::text_input::TextInput;
use crate::components::units::button::Button;
use wasm_bindgen::JsCast;

#[function_component(SimpleForm)]
pub fn simple_form() -> Html {
    html! {
        <form>
            <TextInput name="username" placeholder="Username" />
            <Button label="Submit" />
        </form>
    }
}