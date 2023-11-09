// return all the routes as /api/<route-path>

use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{accept_file_upload, download_file, file_upload_form, get_file};

// the app is moved here to allow sharing across test modules
pub fn app() -> Router {
    Router::new()
        .route("/", post(accept_file_upload).get(file_upload_form))
        .route("/api/download", get(download_file))
        .route("/api/file", get(get_file))
}
