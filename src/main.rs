use actix_web::{App, HttpServer};

mod models;
mod routers;
mod services;
use actix_cors::Cors;
use routers::todolist::*;
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api = HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);
        App::new().wrap(cors)
            .app_data(json!({

                 "serialize_options":{
                    "indent": "",
                    "serialize_null": false,
                    "serialize_recursively": false
                 }
            }))
            .configure(config)
    });

    let port: i32 = 8082;

    let api = api
        .bind(format!("127.0.0.1:{}", port))
        .expect("🧨 Não foi possivel conectar...");

    println!("Conectado com sucesso! ✨✅ http://localhost:{}/hey", port);

    api.run().await
}
