pub mod db;
pub mod ff_core;
pub mod questions;
pub mod restApi;

extern crate serde;
extern crate serde_json;
#[cfg(test)]
#[path = "../test/test.rs"]
pub mod test;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use restApi::restQuestion;

use std::sync::Mutex;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(restQuestion::AppState {
        tickets: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(restQuestion::post_Answer)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
