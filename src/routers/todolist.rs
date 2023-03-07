use crate::models::to_do_crate_command::ToDoCrateCommand;
use crate::models::to_do_update_command::ToDoUpdateCommand;
use actix_web::{get, patch, post, web, HttpResponse, Responder};

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("")]
async fn create(req_body: web::Json<ToDoCrateCommand>) -> impl Responder {
    if let Err(mensagem) = req_body.validate() {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!("{{ \"message\":{} }}", mensagem));
    }

    HttpResponse::Ok().json(req_body)
}

#[patch("")]
async fn update(req_body: web::Json<ToDoUpdateCommand>) -> impl Responder {
    let valores = req_body.into_inner();

    if let Err(mensagem) = valores.validate() {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!("{{ \"message\":{} }}", mensagem));
    }


    return HttpResponse::Ok().json(ToDoUpdateCommand::new(valores.name, valores.deadline, valores.status)); 
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .service(create)
            .service(hello)
            .service(echo)
            .service(update)
            .route("/hey", web::get().to(manual_hello)),
    );
}
