use actix_web::{App, HttpServer};

mod routers;
mod models;
use routers::todolist::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api = HttpServer::new(|| App::new().configure(config));

    let port: i32 = 8082;

    let api = api
        .bind(format!("127.0.0.1:{}", port))
        .expect("🧨 Não foi possivel conectar...");

    println!("Conectado com sucesso! ✨✅ http://localhost:{}/hey", port);

    api.run().await
}
