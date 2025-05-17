use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, Json};
use generator::{generate_equation, Difficulty};
use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::{dto::EquationDto, state::StateStruct};

mod generator;

#[axum::debug_handler]
pub async fn generate(
    Path(difficulty): Path<String>,
    State(state): State<Arc<Mutex<StateStruct>>>
) -> 
    Result<(StatusCode, Json<EquationDto>), (StatusCode, Json<Value>)>
{
    let diff = match &*difficulty.trim().to_lowercase() {
        "easy" => Difficulty::Easy,
        "mid" => Difficulty::Mid,
        "hard" => Difficulty::Hard,
        _ => {
            return Err((
                StatusCode::BAD_GATEWAY,
                Json(json!({
                    "message": "Unknown level of difficulty"
                }))
            ));
        }
    };

    let mut state = state.lock().await;
    let equation = generate_equation(&mut state, diff);
    Ok((StatusCode::OK, Json(equation)))
}