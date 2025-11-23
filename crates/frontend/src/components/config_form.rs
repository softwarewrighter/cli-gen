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

#[derive(Serialize, Deserialize, Clone)]
pub struct BuildInfo {
    pub commit_sha: String,
    pub build_time: String,
    pub build_host: String,
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
    let build_info = use_state(|| None::<BuildInfo>);

    // Fetch build info on mount
    {
        let build_info = build_info.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(info) = fetch_build_info().await {
                    build_info.set(Some(info));
                }
            });
            || ()
        });
    }

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
            let textarea = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
            long_description.set(textarea.value());
        })
    };

    let on_copyright_change = {
        let copyright = copyright.clone();
        Callback::from(move |e: InputEvent| {
            let textarea = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
            copyright.set(textarea.value());
        })
    };

    let on_license_change = {
        let license = license.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            if let Some(select) = target.dyn_ref::<web_sys::HtmlSelectElement>() {
                let value = select.value();
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
        <div class="min-h-screen flex flex-col bg-gradient-to-br from-gray-50 to-gray-100">
            // Header
            <header class="bg-white shadow-sm border-b border-gray-200">
                <div class="max-w-4xl mx-auto px-4 py-3 flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <svg class="w-8 h-8 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                        </svg>
                        <h1 class="text-xl font-bold text-gray-900">{ "CLI Code Generator" }</h1>
                    </div>
                </div>
            </header>

            // Main Content
            <main class="flex-1 max-w-4xl w-full mx-auto px-4 py-8">
                <div class="bg-white rounded-lg shadow-md border border-gray-200 p-6 md:p-8">
                    <div class="mb-6">
                        <h2 class="text-2xl font-semibold text-gray-900 mb-2">{ "Configure Your CLI" }</h2>
                        <p class="text-gray-600">{ "Fill in the details below to generate your command-line interface boilerplate code." }</p>
                    </div>

                    <form class="space-y-6">
                        // CLI Name
                        <div>
                            <label for="name" class="block text-sm font-medium text-gray-700 mb-1">
                                { "CLI Name" }
                                <span class="text-red-500">{ "*" }</span>
                            </label>
                            <input
                                type="text"
                                id="name"
                                value={(*name).clone()}
                                oninput={on_name_change}
                                placeholder="my-awesome-cli"
                                class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition"
                            />
                        </div>

                        // Short Description
                        <div>
                            <label for="short_description" class="block text-sm font-medium text-gray-700 mb-1">
                                { "Short Description" }
                            </label>
                            <input
                                type="text"
                                id="short_description"
                                value={(*short_description).clone()}
                                oninput={on_short_desc_change}
                                placeholder="A brief one-line description of your CLI tool"
                                class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition"
                            />
                        </div>

                        // Long Description
                        <div>
                            <label for="long_description" class="block text-sm font-medium text-gray-700 mb-1">
                                { "Long Description / Help Text" }
                            </label>
                            <textarea
                                id="long_description"
                                value={(*long_description).clone()}
                                oninput={on_long_desc_change}
                                rows="6"
                                placeholder="Provide a detailed description of your CLI tool. This will be displayed when users run --help."
                                class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition resize-y"
                            />
                        </div>

                        // Copyright
                        <div>
                            <label for="copyright" class="block text-sm font-medium text-gray-700 mb-1">
                                { "Copyright Notice" }
                            </label>
                            <textarea
                                id="copyright"
                                value={(*copyright).clone()}
                                oninput={on_copyright_change}
                                rows="3"
                                placeholder="Copyright (c) 2025 Your Name. All rights reserved."
                                class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition resize-y"
                            />
                        </div>

                        // License
                        <div>
                            <label for="license" class="block text-sm font-medium text-gray-700 mb-1">
                                { "License" }
                            </label>
                            <select
                                id="license"
                                onchange={on_license_change}
                                class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition bg-white"
                            >
                                <option value="MIT" selected={matches!(&*license, LicenseType::MIT)}>{ "MIT License" }</option>
                                <option value="Apache-2.0" selected={matches!(&*license, LicenseType::Apache2)}>{ "Apache License 2.0" }</option>
                                <option value="GPL-3.0" selected={matches!(&*license, LicenseType::GPL3)}>{ "GNU GPL 3.0" }</option>
                                <option value="Custom" selected={matches!(&*license, LicenseType::Custom(_))}>{ "Custom License" }</option>
                            </select>
                        </div>

                        // Checkboxes
                        <div class="space-y-3 pt-2">
                            <div class="flex items-center">
                                <input
                                    type="checkbox"
                                    id="version_support"
                                    checked={*version_support}
                                    onchange={on_version_support_change}
                                    class="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                                />
                                <label for="version_support" class="ml-3 text-sm font-medium text-gray-700">
                                    { "Include Version Support " }
                                    <span class="text-gray-500 font-normal">{ "(-V, --version)" }</span>
                                </label>
                            </div>

                            <div class="flex items-center">
                                <input
                                    type="checkbox"
                                    id="help_support"
                                    checked={*help_support}
                                    onchange={on_help_support_change}
                                    class="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-2 focus:ring-blue-500"
                                />
                                <label for="help_support" class="ml-3 text-sm font-medium text-gray-700">
                                    { "Include Help Support " }
                                    <span class="text-gray-500 font-normal">{ "(-h, --help)" }</span>
                                </label>
                            </div>
                        </div>

                        // Generate Button
                        <div class="pt-4">
                            <button
                                type="button"
                                onclick={on_generate}
                                class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-6 rounded-md shadow-sm transition duration-150 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                            >
                                { "Generate Code" }
                            </button>
                        </div>
                    </form>

                    // Status Message
                    if !(*generation_status).is_empty() {
                        <div class={format!("mt-6 p-4 rounded-md {}",
                            if (*generation_status).starts_with("Success") {
                                "bg-green-50 border border-green-200 text-green-800"
                            } else {
                                "bg-red-50 border border-red-200 text-red-800"
                            }
                        )}>
                            <p class="text-sm font-medium">
                                { (*generation_status).clone() }
                            </p>
                        </div>
                    }
                </div>
            </main>

            // Footer
            <footer class="bg-white border-t border-gray-200 mt-auto">
                <div class="max-w-4xl mx-auto px-4 py-4">
                    <div class="flex flex-col md:flex-row justify-between items-center gap-2 text-xs text-gray-600">
                        <div class="flex items-center gap-4">
                            <span>{ "© 2025 CLI Code Generator" }</span>
                            <a href="https://opensource.org/licenses/MIT" target="_blank" rel="noopener" class="hover:text-blue-600 transition">
                                { "MIT License" }
                            </a>
                        </div>
                        if let Some(info) = &*build_info {
                            <div class="flex items-center gap-4 font-mono">
                                <span title="Commit SHA">
                                    { format!("#{}", &info.commit_sha[..7]) }
                                </span>
                                <span title="Build Time">
                                    { &info.build_time }
                                </span>
                                <span title="Build Host">
                                    { &info.build_host }
                                </span>
                            </div>
                        }
                    </div>
                </div>
            </footer>
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

async fn fetch_build_info() -> Result<BuildInfo, String> {
    let response = Request::get("/api/build-info")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response
            .json::<BuildInfo>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to fetch build info: {}", response.status()))
    }
}
