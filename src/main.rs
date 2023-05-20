use actix_web::{App, get, HttpResponse, HttpServer, Responder};

#[get("/board")]
async fn board() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(board))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
