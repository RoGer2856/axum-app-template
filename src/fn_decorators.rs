use std::future::Future;

use axum::{http::StatusCode, response::IntoResponse};
use axum_helpers::auth::LoginInfoExtractor;

use crate::model::login_info::LoginInfo;

pub async fn check_required_role<FutureType: Future<Output = impl IntoResponse>>(
    required_role: &str,
    f: impl FnOnce(LoginInfoExtractor<LoginInfo>) -> FutureType,
    LoginInfoExtractor(login_info): LoginInfoExtractor<LoginInfo>,
) -> Result<impl IntoResponse, StatusCode> {
    if login_info.role == required_role {
        Ok(f(LoginInfoExtractor(login_info)).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
