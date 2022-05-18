// use actix::{Actor, AsyncContext, Context, StreamHandler, Addr};
// use actix_web::{App, HttpResponse, HttpServer, get, HttpRequest, web};
// use actix_web_actors::ws;
// use actix_web_actors::ws::{Message, WebsocketContext};
//
// /// 启动服务器
// pub async fn create_app() {
//     let (addr, port) = ("0.0.0.0", "8086");
//
//     let _ = HttpServer::new(move || {
//         App::new()
//             .service(index)
//     })
//         .bind(format!("{}:{}", addr, port))
//         .expect(&format!("Can't bind to port {}", port))
//         .run()
//         .await;
// }
//
// pub struct WsConn {
//     pub nick: String,
// }
//
// impl Actor for WsConn {
//     type Context = WebsocketContext<Self>;
//
//     /// 连接上
//     fn started(&mut self, _: &mut Self::Context) {
//         println!("{} join!", self.nick);
//     }
//
//     /// 断开连接
//     fn stopped(&mut self, _: &mut Self::Context) {
//         println!("{} exit!", self.nick);
//     }
// }
//
// impl StreamHandler<Result<Message, ws::ProtocolError>> for WsConn {
//     fn handle(&mut self, item: Result<Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match item {
//             Ok(Message::Text(text)) => {
//                 println!("received: {}", text);
//                 let req = WsRequest::from_str(&text);
//                 req.run_cmd(ctx.address());
//             },
//             Ok(Message::Ping(msg)) => ctx.pong(&msg),
//             Ok(Message::Binary(bin)) => ctx.binary(bin),
//             Ok(Message::Close(reason)) => ctx.close(reason),
//             _ => (),
//         }
//     }
// }
//
// pub struct WsRequest {
//     pub cmd: String,
//     pub data: String,
// }
//
// impl WsRequest {
//     fn new(cmd: &str, data: &str) -> Self {
//         Self {
//             cmd: cmd.to_string(),
//             data: data.to_string(),
//         }
//     }
//     pub fn from_str(text: &str) -> Self {
//         let text = text.trim();
//         let mut splitn_result = text.splitn(2, ".");
//         let cmd = splitn_result.next().unwrap();
//         let data = splitn_result.next();
//         if let Some(data) = data {
//             Self::new(cmd, data)
//         } else {
//             Self::new(cmd, "")
//         }
//     }
//     pub fn run_cmd(self, addr: Addr<WsConn>) {
//         if let Some(call_fn) = CMD_MAP.get(self.cmd.as_str()) {
//             call_fn(addr, self.data);
//         } else {
//             eprintln!("no cmd call [{}]", self.cmd);
//         }
//     }
// }
//
//
// #[get("/ws")]
// async fn index(
//     req: HttpRequest,
//     stream: web::Payload,
// ) -> HttpResponse {
//     let conn = WsConn {
//         nick: "张三".into()
//     };
//     let resp = actix_web_actors::ws::start(conn, &req, stream);
//     match resp {
//         Ok(ret) => ret,
//         Err(e) => e.error_response(),
//     }
// }
//
// #[actix_web::main]
// async fn main() {
//     create_app().await;
// }

// `Actor` is not implemented for `WsConn`

fn main() {

}