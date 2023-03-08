use crate::models::to_do_crate_command::ToDoCrateCommand;
use crate::models::to_do_update_command::ToDoUpdateCommand;
use crate::services::to_do_list_service::{add_item, delete_item, get_items, update_item};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(get_items().unwrap())
}

#[post("")]
async fn create(req_body: web::Json<ToDoCrateCommand>) -> impl Responder {
    if let Err(mensagem) = req_body.validate() {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!("{{ \"message\":{} }}", mensagem));
    }

    let item = add_item(req_body.into_inner());

    HttpResponse::Ok().json(item.unwrap())
}

#[patch("/{id}")]
async fn update(
    id: web::Path<uuid::Uuid>,
    req_body: web::Json<ToDoUpdateCommand>,
) -> impl Responder {
    let valores = req_body.into_inner();

    if let Err(mensagem) = valores.validate() {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!("{{ \"message\":{} }}", mensagem));
    }

    let entity = update_item(id.into_inner(), valores);

    return HttpResponse::Ok().json(entity.unwrap());
}

#[delete("/{id}")]
async fn delete(id: web::Path<uuid::Uuid>) -> impl Responder {
    delete_item(&id);

    return HttpResponse::Ok();
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ToDo")
            .service(create)
            .service(hello)
            .service(update)
            .route("/hey", web::get().to(manual_hello)),
    );
}
