use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

pub struct Wss;

impl Actor for Wss {
  type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Wss {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Text(text)) => ctx.text(text),
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      _ => (),
    }
  }
}