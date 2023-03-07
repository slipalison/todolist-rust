use actix_web::{App, HttpServer};

mod routers;
mod models;
mod services;
use routers::todolist::*;
use serde_json::json;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api = HttpServer::new(|| {
        App::new()
        .app_data(json!({
        
             "serialize_options":{
                "indent": "",
                "serialize_null": false,
                "serialize_recursively": false
             }
        })).configure(config)
    });

    let port: i32 = 8082;

    let api = api
        .bind(format!("127.0.0.1:{}", port))
        .expect("🧨 Não foi possivel conectar...");

    println!("Conectado com sucesso! ✨✅ http://localhost:{}/hey", port);

    api.run().await
}
