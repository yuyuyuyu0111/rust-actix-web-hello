use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("a")]
async fn a() -> impl Responder {
    HttpResponse::Ok().body("aaaaaaaaaaaaaaaaa\naaaaaaaaaaaaaaaaaaaaa")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(a))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
