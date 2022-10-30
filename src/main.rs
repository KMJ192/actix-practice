use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, http::header::{CacheControl, CacheDirective}};
use actix_files::Files;

pub mod render;
use render::{RenderParam, render};

pub mod wasm_builder;
use wasm_builder::wasm_builder;

#[get("/")]
async fn root_page() -> impl Responder {
  HttpResponse::Ok()
    .insert_header(CacheControl(
      vec![
        CacheDirective::NoCache,
        CacheDirective::NoStore,
        CacheDirective::MustRevalidate,
      ]
    ))
    .body(render(RenderParam {
      ssr: String::from("<div>root</div>"),
      meta_data: String::from(""),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  wasm_builder();

  let application = HttpServer::new(|| {
    // let cors = 
    //   Cors::default()
    //     .allowed_origin("*");

    App::new()
      // .wrap(cors)
      .service(root_page)
      .service(Files::new("/src", "./client").index_file("index.js"))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await;

  application
}