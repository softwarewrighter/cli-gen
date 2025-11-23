use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::models::{CliConfig, LicenseType};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub output_dir: String,
}

#[function_component(ConfigForm)]
pub fn config_form() -> Html {
    let config = use_state(CliConfig::default);
    let name = use_state(|| config.name.clone());
    let short_description = use_state(|| config.short_description.clone());
    let long_description = use_state(|| config.long_description.clone());
    let copyright = use_state(|| config.copyright.clone());
    let license = use_state(|| config.license.clone());
    let version_support = use_state(|| config.version_support);
    let help_support = use_state(|| config.help_support);
    let generation_status = use_state(String::new);

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            name.set(input.value());
        })
    };

    let on_short_desc_change = {
        let short_description = short_description.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            short_description.set(input.value());
        })
    };

    let on_long_desc_change = {
        let long_description = long_description.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            long_description.set(input.value());
        })
    };

    let on_copyright_change = {
        let copyright = copyright.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            copyright.set(input.value());
        })
    };

    let on_license_change = {
        let license = license.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            // Use dyn_ref to convert to HtmlSelectElement
            if let Some(select) = target.dyn_ref::<web_sys::HtmlSelectElement>() {
                let value = select.value();
                let license_type = match value.as_str() {
                    "MIT" => LicenseType::MIT,
                    "Apache-2.0" => LicenseType::Apache2,
                    "GPL-3.0" => LicenseType::GPL3,
                    _ => LicenseType::Custom(value),
                };
                license.set(license_type);
            } else if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
                // Fallback for different element types
                let value = input.value();
                let license_type = match value.as_str() {
                    "MIT" => LicenseType::MIT,
                    "Apache-2.0" => LicenseType::Apache2,
                    "GPL-3.0" => LicenseType::GPL3,
                    _ => LicenseType::Custom(value),
                };
                license.set(license_type);
            }
        })
    };

    let on_version_support_change = {
        let version_support = version_support.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            version_support.set(input.checked());
        })
    };

    let on_help_support_change = {
        let help_support = help_support.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            help_support.set(input.checked());
        })
    };

    let on_generate = {
        let config = config.clone();
        let name = name.clone();
        let short_description = short_description.clone();
        let long_description = long_description.clone();
        let copyright = copyright.clone();
        let license = license.clone();
        let version_support = version_support.clone();
        let help_support = help_support.clone();
        let generation_status = generation_status.clone();

        Callback::from(move |_| {
            let new_config = CliConfig {
                name: (*name).clone(),
                short_description: (*short_description).clone(),
                long_description: (*long_description).clone(),
                copyright: (*copyright).clone(),
                license: (*license).clone(),
                version_support: *version_support,
                help_support: *help_support,
            };

            config.set(new_config.clone());

            // Call the API to generate code
            let generation_status_clone = generation_status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match generate_code(new_config).await {
                    Ok(response) => {
                        generation_status_clone.set(format!(
                            "Success! Code generated to: {}",
                            response.output_dir
                        ));
                    }
                    Err(err) => {
                        generation_status_clone.set(format!("Error: {}", err));
                    }
                }
            });
        })
    };

    html! {
        <div class="container">
            <h1>{ "CLI Code Generator" }</h1>
            <form class="config-form">
                <div class="form-group">
                    <label for="name">{ "CLI Name:" }</label>
                    <input
                        type="text"
                        id="name"
                        value={(*name).clone()}
                        oninput={on_name_change}
                        class="form-control"
                    />
                </div>

                <div class="form-group">
                    <label for="short_description">{ "Short Description:" }</label>
                    <input
                        type="text"
                        id="short_description"
                        value={(*short_description).clone()}
                        oninput={on_short_desc_change}
                        class="form-control"
                    />
                </div>

                <div class="form-group">
                    <label for="long_description">{ "Long Description:" }</label>
                    <textarea
                        id="long_description"
                        value={(*long_description).clone()}
                        oninput={on_long_desc_change}
                        class="form-control"
                    />
                </div>

                <div class="form-group">
                    <label for="copyright">{ "Copyright:" }</label>
                    <input
                        type="text"
                        id="copyright"
                        value={(*copyright).clone()}
                        oninput={on_copyright_change}
                        class="form-control"
                    />
                </div>

                <div class="form-group">
                    <label for="license">{ "License:" }</label>
                    <select
                        id="license"
                        onchange={on_license_change}
                        class="form-control"
                    >
                        <option value="MIT" selected={matches!(&*license, LicenseType::MIT)}>{ "MIT" }</option>
                        <option value="Apache-2.0" selected={matches!(&*license, LicenseType::Apache2)}>{ "Apache 2.0" }</option>
                        <option value="GPL-3.0" selected={matches!(&*license, LicenseType::GPL3)}>{ "GPL 3.0" }</option>
                        <option value="Custom" selected={matches!(&*license, LicenseType::Custom(_))}>{ "Custom" }</option>
                    </select>
                </div>

                <div class="form-group">
                    <label class="checkbox">
                        <input
                            type="checkbox"
                            checked={*version_support}
                            onchange={on_version_support_change}
                        />
                        { " Include Version Support (-V/--version)" }
                    </label>
                </div>

                <div class="form-group">
                    <label class="checkbox">
                        <input
                            type="checkbox"
                            checked={*help_support}
                            onchange={on_help_support_change}
                        />
                        { " Include Help Support (-h/--help)" }
                    </label>
                </div>

                <button
                    type="button"
                    onclick={on_generate}
                    class="btn btn-primary"
                >
                    { "Generate Code" }
                </button>
            </form>

            if !(*generation_status).is_empty() {
                <div class="alert alert-info">
                    { (*generation_status).clone() }
                </div>
            }
        </div>
    }
}

async fn generate_code(config: CliConfig) -> Result<ApiResponse, String> {
    let request_body = serde_json::to_string(&config).map_err(|e| e.to_string())?;

    let response = Request::post("/api/generate")
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response
            .json::<ApiResponse>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Request failed with status: {}", response.status()))
    }
}
