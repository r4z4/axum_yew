use reqwasm::http::Request;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ApiLoginResponse {
    pub id: i32,
    pub username: String,
    pub token: String,
}

pub async fn login_user(username: String, password: String) -> ApiLoginResponse {
    let body = json!({
        "username": username,
        "password": password
    });
    let response = Request::post("http://localhost:3000/users/login")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiLoginResponse>()
        .await
        .unwrap();

    response
}