use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use actix_web::{get, post, web, HttpResponse, Responder};

// 新しい記事を作成するためのDTO
#[derive(Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub content: String,
    pub author: Option<String>,
}

// Articleテーブルに対応
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Article {
    // 全ての記事を取得
    pub async fn find_all(pool: &Pool<Postgres>) -> Result<Vec<Article>, sqlx::Error> {
        sqlx::query_as::<_, Article>("SELECT * FROM articles ORDER BY id")
            .fetch_all(pool)
            .await
    }

    // IDで記事を取得
    pub async fn find_by_id(id: i32, pool: &Pool<Postgres>) -> Result<Option<Article>, sqlx::Error> {
        sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    // 新しい記事を作成
    pub async fn create(
        new_article: CreateArticle,
        pool: &Pool<Postgres>,
    ) -> Result<Article, sqlx::Error> {
        sqlx::query_as::<_, Article>(
            "INSERT INTO articles (title, content, author, created_at) 
             VALUES ($1, $2, $3, CURRENT_TIMESTAMP) 
             RETURNING *",
        )
        .bind(new_article.title)
        .bind(new_article.content)
        .bind(new_article.author)
        .fetch_one(pool)
        .await
    }
}

// 全ての記事を取得するエンドポイント
#[get("/articles")]
async fn get_articles(db: web::Data<Pool<Postgres>>) -> impl Responder {
    match Article::find_all(&db).await {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(e) => {
            eprintln!("Failed to get articles: {}", e);
            HttpResponse::InternalServerError().body("Failed to get articles")
        }
    }
}

// IDで記事を取得するエンドポイント
#[get("/articles/{id}")]
async fn get_article(path: web::Path<i32>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    let id = path.into_inner();

    match Article::find_by_id(id, &db).await {
        Ok(Some(article)) => HttpResponse::Ok().json(article),
        Ok(None) => HttpResponse::NotFound().body(format!("No article found with id: {}", id)),
        Err(e) => {
            eprintln!("Failed to get article {}: {}", id, e);
            HttpResponse::InternalServerError().body(format!("Failed to get article {}", id))
        }
    }
}

// 新しい記事を作成するエンドポイント
#[post("/articles")]
async fn create_article(
    new_article: web::Json<CreateArticle>,
    db: web::Data<Pool<Postgres>>,
) -> impl Responder {
    match Article::create(new_article.into_inner(), &db).await {
        Ok(article) => HttpResponse::Created().json(article),
        Err(e) => {
            eprintln!("Failed to create article: {}", e);
            HttpResponse::InternalServerError().body("Failed to create article")
        }
    }
}