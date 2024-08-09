use std::{collections::BTreeMap, sync::Arc, time::Duration};

use axum::{
    async_trait,
    error_handling::HandleErrorLayer,
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use axum_helpers::{
    app::AxumAppState,
    auth::{AuthError, AuthHandler, AuthLayer},
};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::{
    model::login_info::{LoginInfo, StoredLoginInfo},
    syn::{arc_rw_lock_new, ArcRwLock},
};

const ACCESS_TOKEN_EXPIRATION_TIME_DURATION: Duration = Duration::from_secs(60);

#[derive(Clone)]
pub struct AppState {
    secret: Vec<u8>,
    pub logins: ArcRwLock<BTreeMap<LoginName, StoredLoginInfo>>,
}

#[derive(Debug, Clone)]
pub struct AccessToken(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LoginName(pub String);

#[derive(Debug, Serialize, Deserialize)]
struct UserLoginClaims {
    sub: String,
    role: String,
    exp: usize,
}

impl AppState {
    pub fn new(secret: impl Into<Vec<u8>>) -> Self {
        Self {
            secret: secret.into(),
            logins: arc_rw_lock_new(BTreeMap::new()),
        }
    }

    pub fn login(
        &mut self,
        loginname: impl Into<String>,
        _password: impl Into<String>,
    ) -> Result<(Duration, AccessToken), ()> {
        let loginname = loginname.into();
        let role = match loginname.as_str() {
            "admin" => "admin",
            _ => "regular",
        }
        .into();

        let jwt = self.create_jwt_for_user(&loginname, &role)?;
        let access_token = AccessToken(jwt);

        let login_info = StoredLoginInfo {
            loginname,
            role,
            logged_in: true,
        };

        self.logins
            .write()
            .entry(LoginName(login_info.loginname.clone()))
            .and_modify(|login_info: &mut StoredLoginInfo| {
                login_info.logged_in = true;
            })
            .or_insert_with(|| StoredLoginInfo {
                loginname: login_info.loginname.clone(),
                role: login_info.role.clone(),
                logged_in: true,
            });

        log::info!("User logged in, loginname = '{}'", login_info.loginname);

        Ok((ACCESS_TOKEN_EXPIRATION_TIME_DURATION, access_token))
    }

    pub fn logout(&mut self, login_info: &Arc<LoginInfo>) {
        if let Some(login_info) = self
            .logins
            .write()
            .get_mut(&LoginName(login_info.loginname.clone()))
        {
            log::info!("User logged out, loginname = '{}'", login_info.loginname);
            login_info.logged_in = false;
        }
    }

    fn create_jwt_for_user(
        &self,
        loginname: impl Into<String>,
        role: impl Into<String>,
    ) -> Result<String, ()> {
        let expiration_time = std::time::SystemTime::now() + ACCESS_TOKEN_EXPIRATION_TIME_DURATION;
        let exp = expiration_time
            .duration_since(std::time::UNIX_EPOCH)
            .inspect_err(|e| log::error!("create_jwt_for_user, exp calculation, error = {e}"))
            .map_err(|_| ())?
            .as_secs();

        jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512),
            &UserLoginClaims {
                sub: loginname.into(),
                role: role.into(),
                exp: exp as usize,
            },
            &jsonwebtoken::EncodingKey::from_secret(&self.secret),
        )
        .inspect_err(|e| log::error!("create_jwt_for_user, encode, error = {e}"))
        .map_err(|_| ())
    }

    fn decode_user_jwt(&self, token: &str) -> Result<UserLoginClaims, ()> {
        jsonwebtoken::decode::<UserLoginClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(&self.secret),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512),
        )
        .map(|token_data| token_data.claims)
        .inspect_err(|e| log::error!("decode_user_jwt, decode, error = {e}"))
        .map_err(|_| ())
    }
}

#[async_trait]
impl AuthHandler<LoginInfo> for AppState {
    async fn verify_access_token(&mut self, access_token: &str) -> Result<LoginInfo, AuthError> {
        let user_login_claims = self
            .decode_user_jwt(access_token)
            .map_err(|_| AuthError::InvalidAccessToken)?;

        self.logins
            .read()
            .get(&LoginName(user_login_claims.sub.clone()))
            .ok_or_else(|| AuthError::InvalidAccessToken)
            .and_then(|login_info| {
                if login_info.logged_in {
                    Ok(login_info.into())
                } else {
                    Err(AuthError::InvalidAccessToken)
                }
            })
    }

    async fn update_access_token(
        &mut self,
        _access_token: &str,
        login_info: &Arc<LoginInfo>,
    ) -> Result<(String, Duration), AuthError> {
        self.logins
            .read()
            .get(&LoginName(login_info.loginname.clone()))
            .ok_or_else(|| AuthError::InvalidAccessToken)
            .and_then(|login_info| {
                if login_info.logged_in {
                    let access_token = self
                        .create_jwt_for_user(&login_info.loginname, &login_info.role)
                        .map_err(|_| AuthError::Internal)?;
                    Ok((access_token, ACCESS_TOKEN_EXPIRATION_TIME_DURATION))
                } else {
                    Err(AuthError::InvalidAccessToken)
                }
            })
    }

    async fn invalidate_access_token(&mut self, _access_token: &str, login_info: &Arc<LoginInfo>) {
        self.logout(login_info);
    }
}

impl AxumAppState for AppState {
    fn routes(&self) -> Router {
        Router::new()
            .nest_service("/public", ServeDir::new("public"))
            .route("/", get(crate::endpoints::index))
            .route("/login", get(crate::endpoints::login))
            .route("/api/login", post(crate::endpoints::api::login))
            .route("/api/logout", post(crate::endpoints::api::logout))
            .route(
                "/api/seen-users",
                get(crate::endpoints::api::get_seen_users),
            )
            .route(
                "/api/seen-users/:index",
                get(crate::endpoints::api::get_seen_user),
            )
            .route("/api/create-uuid-v4", get(create_uuid_v4))
            .route(
                "/api/echo/:this/and/:that",
                get(crate::endpoints::api::echo_this_and_that),
            )
            .route("/api/echo-path", get(crate::endpoints::api::echo_path))
            .route(
                "/api/echo-query-params",
                get(crate::endpoints::api::echo_query_params),
            )
            .route(
                "/api/echo-parsed-query-params",
                get(crate::endpoints::api::echo_parsed_query_params),
            )
            .route(
                "/api/echo-uuid-in-path/:uuid",
                get(crate::endpoints::api::echo_uuid_in_path),
            )
            .route_layer(AuthLayer::new(self.clone()))
            // use this layer to change the body limit, the default is 2MB
            // .layer(DefaultBodyLimit::disable())
            .layer(DefaultBodyLimit::max(2 * 1024 * 1024))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(handle_timeout_error))
                    .timeout(Duration::from_secs(30)),
            )
            .with_state(self.clone())
    }
}

async fn handle_timeout_error(err: tower::BoxError) -> StatusCode {
    if err.is::<tower::timeout::error::Elapsed>() {
        StatusCode::REQUEST_TIMEOUT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn create_uuid_v4() -> String {
    Uuid::new_v4().as_hyphenated().to_string()
}
