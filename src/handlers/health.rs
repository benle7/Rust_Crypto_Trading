use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
