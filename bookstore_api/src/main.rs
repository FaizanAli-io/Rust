use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
struct Book {
    id: i32,
    title: String,
    author: String,
    published: bool,
    created_at: NaiveDateTime,
}

#[derive(Deserialize)]
struct NewBook {
    title: String,
    author: String,
    published: Option<bool>,
}

struct AppState {
    db: PgPool,
}

async fn get_books(data: web::Data<AppState>) -> impl Responder {
    let result: Result<_, _> = sqlx::query_as::<_, Book>("SELECT * FROM books ORDER BY id")
        .fetch_all(&data.db)
        .await;

    match result {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch books")
        }
    }
}

async fn add_book(book: web::Json<NewBook>, data: web::Data<AppState>) -> impl Responder {
    let published: bool = book.published.unwrap_or(false);

    let result: Result<_, _> = sqlx::query_as::<_, Book>(
        "INSERT INTO books (title, author, published) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&book.title)
    .bind(&book.author)
    .bind(published)
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(saved) => HttpResponse::Created().json(saved),
        Err(e) => {
            eprintln!("Insert error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to insert book")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url: &'static str = "postgres://bookuser:bookpass@localhost/bookstore";

    let pool: sqlx::Pool<sqlx::Postgres> =
        PgPool::connect(db_url).await.expect("DB connection failed");

    let state: web::Data<AppState> = web::Data::new(AppState { db: pool });

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/books", web::get().to(get_books))
            .route("/books", web::post().to(add_book))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
