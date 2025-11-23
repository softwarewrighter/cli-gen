use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use clap::Parser;
use cli_codegen_backend::{codegen::generators::CodeGenerator, models::config::CliConfig};
use serde_json::Value;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Parser)]
#[clap(
    name = "cli-codegen-server",
    version = "1.0",
    about = "CLI Code Generator Server"
)]
struct Args {
    /// Port to run the server on
    #[clap(short = 'p', long = "port", default_value = "3000")]
    port: u16,
}

// Application state
#[derive(Clone)]
struct AppState {
    config: CliConfig,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    start_web_server(args.port).await
}

async fn start_web_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize default configuration
    let initial_config = CliConfig::default();
    let app_state = AppState {
        config: initial_config,
    };

    // Build our application with a route
    let app = Router::new()
        // API routes
        .route("/api/config", get(get_config).post(update_config))
        .route("/api/generate", post(api_generate))
        // Serve static assets (favicon, etc.) from static directory
        .nest_service("/static", ServeDir::new("static"))
        // Serve frontend from the index directory (symlinked to frontend/dist)
        .nest_service("/", ServeDir::new("index"))
        .with_state(app_state);

    // Run our application
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    println!(
        "CLI Code Generator is running on http://{}",
        listener.local_addr()?
    );

    axum::serve(listener, app).await?;

    Ok(())
}

// API handlers
async fn get_config(State(state): State<AppState>) -> Json<CliConfig> {
    Json(state.config)
}

async fn update_config(
    State(_state): State<AppState>,
    Json(config): Json<CliConfig>,
) -> Result<Json<CliConfig>, StatusCode> {
    // In a real app, we would update the state here
    // For now, we'll just return the config as is
    Ok(Json(config))
}

async fn api_generate(Json(config): Json<CliConfig>) -> Result<Json<Value>, StatusCode> {
    // Validate the configuration
    if let Err(_e) = CodeGenerator::validate_config(&config) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Generate code to the ./work directory with a sanitized project name
    let sanitized_name = config
        .name
        .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");
    let output_dir = format!("../../work/{}", sanitized_name);

    // Create work directory if it doesn't exist
    std::fs::create_dir_all(&output_dir).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Err(_e) = CodeGenerator::generate_files(&config, &output_dir) {
        eprintln!("Generation error: {}", _e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(serde_json::json!({
        "status": "success",
        "output_dir": output_dir,
        "message": format!("CLI code generated successfully for '{}'", config.name)
    })))
}
