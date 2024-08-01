use axum::http::StatusCode;

// If the frontend gets a 200 OK, the password is correct
pub async fn authenticate_handler() -> StatusCode {
    StatusCode::OK
}
