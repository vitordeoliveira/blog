use axum::response::IntoResponse;

use crate::{error::Result, view::root as rootView};

pub async fn root() -> Result<impl IntoResponse> {
    rootView()
}
