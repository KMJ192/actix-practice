use std::process::Command;

use actix_web::{web, get, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use actix_web_actors::ws;
// use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};

pub mod render;
pub mod wss;
pub mod file_watcher;
use render::{RenderParam, render};
use wss::Wss;
use file_watcher::file_watcher;


#[get("/")]
async fn root_page() -> impl Responder {
  HttpResponse::Ok().body(render(RenderParam {
    ssr: String::from("<div>root</div>"),
    meta: String::from(""),
  }))
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  ws::start(Wss {}, &req, stream)
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

  let f = file_watcher();

  let application = HttpServer::new(|| {
    App::new()
      .service(root_page)
      .service(Files::new("/src", "./client").index_file("index.js"))
      .route("/ws/", web::get().to(index))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await;

  application
}