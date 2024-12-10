use crate::db::models::UserRecord;
use crate::db::mongodb::MongoDB;
use crate::db::traits::users::QueryUserRecords;
use crate::handlers::entities::{LoginRequest, RegisterRequest};
use crate::handlers::utils;
use actix_web::{http::header::ContentType, post, web, HttpResponse};

const STRONG_PASSWORD_LEN: usize = 32;

#[post("/register")]
pub async fn register(
    db_client: web::Data<MongoDB>,
    request: web::Json<RegisterRequest>,
) -> HttpResponse {
    let username = request.username.clone();
    match db_client.find_user(username.as_str()).await {
        Ok(Some(_)) => HttpResponse::BadRequest().body("Username already exists"),
        Ok(None) => {
            let password = request
                .password
                .clone()
                .unwrap_or(utils::generate_random_string(STRONG_PASSWORD_LEN));
            let user = UserRecord {
                username: username.clone(),
                hashed_password: utils::sha256(password.as_str()),
            };
            match db_client.add_user(&user).await {
                Ok(_) => {
                    HttpResponse::Ok()
                        .content_type(ContentType::json())
                        .json(serde_json::json!(
                            {
                                "username": username,
                                "password": password.clone(),
                            }
                        ))
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to register user"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to register user"),
    }
}

#[post("/login")]
pub async fn login(
    db_client: web::Data<MongoDB>,
    request: web::Json<LoginRequest>,
) -> HttpResponse {
    let username = request.username.clone();
    match db_client.find_user(username.as_str()).await {
        Ok(Some(user)) => {
            let hashed_password = utils::sha256(&request.password.clone());
            if user.hashed_password != hashed_password {
                return HttpResponse::BadRequest().body("Invalid login details");
            }
            let jwt = match utils::create_jwt(username).await {
                Ok(token) => token,
                Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error"),
            };
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(serde_json::json!({ "jwt": jwt }))
        }
        _ => HttpResponse::Unauthorized().body("Invalid login details"),
    }
}
