use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use crate::article::{create_article, get_article, get_articles};

mod article;
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
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = get_db_pool(&database_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Compress::default())
            .service(index)
            .service(a)
            .service(get_articles)
            .service(get_article)
            .service(create_article)
    })
    .bind("127.0.0.1:5000")?
    .bind("100.94.44.127:5000")?
    .run()
    .await
}

async fn get_db_pool(database_url: &String) -> Pool<Postgres> {
    sqlx::PgPool::connect(&database_url).await.unwrap()
}


