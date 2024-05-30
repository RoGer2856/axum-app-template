#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginRequest {
    pub loginname: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub loginname: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EchoThisAndThatResponse {
    pub this: String,
    pub that: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EchoPathResponse {
    pub path: String,
}
