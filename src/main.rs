mod api;

use api::task::{get_task, get_hello};
use actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
   std::env::set_var("RESULT_LOG", "debug");
   std::env::set_var("RUST_BACKTRACE", "1");
   env_logger::init();

   HttpServer::new(move || {
   let logger = Logger::default();
    App::new()
    .wrap(logger)
    .service(get_hello)
   })
   .bind(("127.0.0.1", 80))?
    .run()
    .await
}
// https://www.youtube.com/watch?v=L8tWKqSMKUI&list=PL2q9pua8FpiUwVll2Alk8W_d3eN8IxJjO