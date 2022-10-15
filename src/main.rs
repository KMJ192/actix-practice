use std::process::Command;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, http::header::{CacheControl, CacheDirective}};
use actix_files::Files;

pub mod render;
use render::{RenderParam, render};

#[get("/")]
async fn root_page() -> impl Responder {
  HttpResponse::Ok()
    .insert_header(CacheControl(
      vec![
        CacheDirective::NoCache,
        CacheDirective::NoStore,
        CacheDirective::MustRevalidate,
        // CacheDirective::MaxAge(0)
      ]
    ))
    .body(render(RenderParam {
      ssr: String::from("<div>root</div>"),
      meta: String::from(""),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  if cfg!(target_os = "windows") {
    Command::new("cmd")
      .arg("dev.sh")
      .status()
      .expect("failed to execute process in window");
  } else {
    Command::new("sh")
      .arg("dev.sh")
      .status()
      .expect("failed to execute process in linux");
  };

  let application = HttpServer::new(|| {
    App::new()
      .service(root_page)
      .service(Files::new("/src", "./client").index_file("index.js"))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await;

  application
}