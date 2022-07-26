use crate::questions::question::Answer;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::*;
use actix_web::{
    delete, get, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
    ResponseError,
};
use std::fmt::Display;
use std::sync::Mutex;
pub struct AppState {
   pub  tickets: Mutex<Vec<Answer>>,
}
impl Responder for Answer {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

#[post("/question")]
async fn post_Answer(req: web::Json<Answer>, data: web::Data<AppState>) -> impl Responder {
    let answer_ = Answer::new(req.points, &String::from(&req.answer));
    print!("ANSEWER CAL");

    let mut Answers = data.tickets.lock().unwrap();

    let response = serde_json::to_string(&answer_).unwrap();

    //Answers.push(new_Answer);
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}
