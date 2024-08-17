use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::Query;
use axum_helpers::auth::{AccessTokenResponse, AuthLogoutResponse, LoginInfoExtractor};
use serde_json::json;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    fn_decorators::check_required_role,
    messages::{EchoPathResponse, EchoThisAndThatResponse, LoginRequest, LoginResponse},
    model::login_info::LoginInfo,
};

pub async fn login(
    State(mut state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<(StatusCode, AccessTokenResponse, Json<LoginResponse>), StatusCode> {
    let access_token_response = state
        .login(&login_request.loginname, login_request.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        access_token_response,
        Json(LoginResponse {
            loginname: login_request.loginname,
        }),
    ))
}

pub async fn logout(
    LoginInfoExtractor(_login_info): LoginInfoExtractor<LoginInfo>,
) -> Result<AuthLogoutResponse, StatusCode> {
    Ok(AuthLogoutResponse::new(Some("/"), Some("/")))
}

#[fn_decorator::use_decorator(check_required_role("admin"), override_return_type = impl IntoResponse, exact_parameters = [_login_info])]
pub async fn get_seen_users(
    _login_info: LoginInfoExtractor<LoginInfo>,
    state: State<AppState>,
) -> Json<serde_json::Value> {
    log::info!("get_logged_in_users");

    let login_infos = state
        .logins
        .read()
        .iter()
        .map(|(_access_token, login_info)| login_info.clone())
        .collect::<Vec<_>>();

    Json(json!({
        "login_infos": login_infos
    }))
}

#[fn_decorator::use_decorator(check_required_role("admin"), override_return_type = impl IntoResponse, exact_parameters = [_login_info])]
pub async fn get_seen_user(
    _login_info: LoginInfoExtractor<LoginInfo>,
    state: State<AppState>,
    index: Path<u32>,
) -> Result<Json<LoginInfo>, StatusCode> {
    log::info!("get_logged_in_user: index = '{}'", index.0);

    let login_info = state
        .logins
        .read()
        .iter()
        .nth(index.0 as usize)
        .ok_or(StatusCode::NOT_FOUND)?
        .1
        .into();

    Ok(Json(login_info))
}

pub async fn echo_this_and_that(
    Path((this, that)): Path<(String, String)>,
) -> Json<EchoThisAndThatResponse> {
    log::info!("echo_this_and_that: this = '{}', that = '{}'", this, that,);

    Json(EchoThisAndThatResponse { this, that })
}

pub async fn echo_path(uri: Uri) -> Json<EchoPathResponse> {
    log::info!("echo_path: path = '{uri}'");

    Json(EchoPathResponse {
        path: uri.path().to_string(),
    })
}

pub async fn echo_query_params(
    Query(query_params): Query<serde_json::Value>,
) -> Json<serde_json::Value> {
    log::info!("handle_query_params: query_params = '{:?}'", query_params);

    Json(query_params)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ParseQueryParamsParams {
    list: Vec<String>,
    uuid: Uuid,
}

pub async fn echo_parsed_query_params(
    Query(query_params): Query<ParseQueryParamsParams>,
) -> Json<ParseQueryParamsParams> {
    log::info!("parse_query_params: query_params = '{:?}'", query_params);

    Json(query_params)
}

pub async fn echo_uuid_in_path(Path(uuid): Path<Uuid>) -> Json<Uuid> {
    log::info!("echo_uuid_in_path: uuid = '{}'", uuid);

    Json(uuid)
}
