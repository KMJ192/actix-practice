use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;

pub mod render;
use render::render;

#[get("/")]
async fn root_page() -> impl Responder {
  HttpResponse::Ok().body(render(String::from("<div>root</div>"), String::from("")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
