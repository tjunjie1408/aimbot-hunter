use axum::{
    extract::{State, Json},
    routing::{get, post},
    Router,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::{sync::{Arc, Mutex}, fs};
use tower_http::cors::CorsLayer;

use ort::{
    session::{Session, builder::GraphOptimizationLevel},
    value::Value,
};

#[derive(Debug, Deserialize)]
struct ScalerConfig {
    min: [f32; 2],
    scale: [f32; 2],
    threshold: f32,
}

struct AppState {
    model: Mutex<Session>,
    config: ScalerConfig,
}

#[derive(Deserialize)]
struct MousePoint {
    x: f32,
    y: f32,
}

#[derive(Deserialize)]
struct AnalyzeRequest {
    trajectory: Vec<MousePoint>,
}

#[derive(Serialize)]
struct Verdict {
    is_cheater: bool,
    anomaly_score: f32,
    message: String,
    processed_windows: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = ort::init()
        .with_name("aimbot_hunter")
        .commit();
        
    println!(">>> 🦀 RUST AIMBOT HUNTER API (Powered by ONNX Runtime) STARTING...");

    println!(">>> [1/3] Loading Configuration...");
    let config_content = fs::read_to_string("config.json")
        .expect("❌ Failed to read config.json");
    let config: ScalerConfig = serde_json::from_str(&config_content)?;
    
    println!(">>> [2/3] Loading ONNX Brain...");
    
    let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?
        .commit_from_file("model.onnx")?;
        
    println!("✅ AI Model Optimized & Ready!");

    let state = Arc::new(AppState { 
        model: Mutex::new(session), 
        config 
    });

    println!(">>> [3/3] Launching HTTP Server...");
    let app = Router::new()
        .route("/", get(health_check))
        .route("/analyze", post(detect_cheater))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await?;
    println!("🚀 SERVER RUNNING ON http://localhost:3002");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "Rust Aimbot Hunter (ORT Engine) is Online! 🦀"
}

async fn detect_cheater(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AnalyzeRequest>,
) -> impl IntoResponse {
    let points = payload.trajectory;

    if points.len() < 51 {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Trajectory too short."
        }))).into_response();
    }

    let mut normalized_data = Vec::new();
    for i in 1..points.len() {
        let dx = points[i].x - points[i-1].x;
        let dy = points[i].y - points[i-1].y;
        let dx_scaled = (dx - state.config.min[0]) * state.config.scale[0];
        let dy_scaled = (dy - state.config.min[1]) * state.config.scale[1];
        normalized_data.push(vec![dx_scaled, dy_scaled]);
    }

    let time_steps = 50;
    let num_windows = normalized_data.len() - time_steps;
    
    let mut input_flat: Vec<f32> = Vec::with_capacity(num_windows * time_steps * 2);
    for i in 0..num_windows {
        for j in 0..time_steps {
            input_flat.push(normalized_data[i+j][0]);
            input_flat.push(normalized_data[i+j][1]);
        }
    }
    
    
    let shape = vec![num_windows as i64, time_steps as i64, 2];
    let input_tensor = Value::from_array((shape, input_flat.clone())).unwrap();
    
    let mut model = state.model.lock().unwrap(); 
    let outputs = model.run(ort::inputs![input_tensor]).unwrap();
    
    let output_value = &outputs[0];
    let (_, output_data_slice) = output_value.try_extract_tensor::<f32>().unwrap();

    let input_ndarray = ndarray::Array::from_shape_vec((num_windows, time_steps, 2), input_flat).unwrap();
    
    let mut total_mse = 0.0;
    let count = num_windows * time_steps * 2;

    for (a, b) in input_ndarray.iter().zip(output_data_slice.iter()) {
        total_mse += (*a - *b).powi(2);
    }
    let avg_mse = total_mse / count as f32;

    let is_cheater = avg_mse > state.config.threshold;

    Json(Verdict {
        is_cheater,
        anomaly_score: avg_mse,
        processed_windows: num_windows,
        message: if is_cheater { 
            "🚨 CHEATER DETECTED".to_string() 
        } else { 
            "✅ HUMAN PLAYER".to_string() 
        },
    }).into_response()
}