use actix_web::{get, Responder, HttpResponse, HttpResponseBuilder};

#[get("/api/lab/response")]
pub async fn response() -> impl Responder {
  HttpResponse::Ok()
    .body(String::from("Test"))
}