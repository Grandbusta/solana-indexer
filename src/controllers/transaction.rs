use crate::AppState;
use axum::extract::State;

pub async fn get_transactions(
    State(_app_state): State<AppState>
) {

}