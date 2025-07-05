use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Book {
    id: u32,
    title: String,
    author: String,
}

struct AppState {
    books: Mutex<HashMap<u32, Book>>,
}

async fn get_books(data: web::Data<AppState>) -> impl Responder {
    let books = data.books.lock().unwrap();
    let list: Vec<Book> = books.values().cloned().collect();
    HttpResponse::Ok().json(list)
}

async fn add_book(book: web::Json<Book>, data: web::Data<AppState>) -> impl Responder {
    let mut books = data.books.lock().unwrap();
    books.insert(book.id, book.into_inner());
    HttpResponse::Created().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        books: Mutex::new(HashMap::new()),
    });

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
