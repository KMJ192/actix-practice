use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;


#[get("/")]
async fn render() -> impl Responder {
  HttpResponse::Ok().body(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8" />
      <meta http-equiv="X-UA-Compatible" content="IE=edge" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <title>Document</title>
    </head>
    <body>
      <main id="App">123</main>
      <script src="./src/index.js" type="module"></script>
    </body>
    </html>
  "#)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  let application = HttpServer::new(|| {
    App::new()
      .service(render)
      .service(echo)
      .service(Files::new("/src", "./client").index_file("index.js"))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await;

  application
}
