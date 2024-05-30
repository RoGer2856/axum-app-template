#[derive(Clone, serde::Serialize)]
pub struct LoginInfo {
    pub loginname: String,
    pub role: String,
}

#[derive(Clone, serde::Serialize)]
pub struct StoredLoginInfo {
    pub loginname: String,
    pub role: String,
    pub logged_in: bool,
}

impl From<&StoredLoginInfo> for LoginInfo {
    fn from(stored_login_info: &StoredLoginInfo) -> Self {
        Self {
            loginname: stored_login_info.loginname.clone(),
            role: stored_login_info.role.clone(),
        }
    }
}
