use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Serialize, Deserialize};
use web3::transports::Http;
use web3::Web3;

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    password: String,
    address: String,
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn register(req: web::Json<User>) -> HttpResponse {
    // Código para registrar usuario
    HttpResponse::Created().body("Usuario creado")
}

async fn login(req: web::Json<LoginRequest>) -> HttpResponse {
    // Código para iniciar sesión
    HttpResponse::Ok().body("Sesión iniciada")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn get_web3() -> Web3<Http> {
    let transport = Http::new("(link unavailable)").unwrap();
    Web3::new(transport)
}
