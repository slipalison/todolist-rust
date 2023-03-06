use crate::models::to_do_crate_command::ToDoCrateCommand;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/todo")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/todo/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/todo")]
async fn create(req_body: web::Json<ToDoCrateCommand>) -> impl Responder {
   
    if let Err(mensagem) = req_body.validate() {
        return HttpResponse::BadRequest().body(format!("{{ \"message\":{} }}", mensagem))
    }

    HttpResponse::Ok().json(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(hello);
    cfg.service(echo);
    cfg.route("/hey", web::get().to(manual_hello));
}
