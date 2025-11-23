pub mod components;
mod models;

use crate::components::config_form::ConfigForm;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <nav class="navbar">
                <div class="nav-container">
                    <h2 class="nav-title">{ "CLI Code Generator" }</h2>
                </div>
            </nav>
            <main>
                <ConfigForm />
            </main>
        </>
    }
}

// Entry point for WASM - this is what Trunk will call
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
