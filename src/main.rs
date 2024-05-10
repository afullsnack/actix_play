use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

// Implement handlers for requests
#[get("/")]
async fn index() -> impl Responder {
    "Hello, index!"
}

#[derive(Deserialize)]
struct Info {
    query: String,
}

#[get("/{name}")]
async fn hello(name: web::Path<String>, iden: web::Query<Info>) -> impl Responder {
    let info = iden.into_inner();
    println!("The query param: {}", info.query);

    if name.contains("mike") {
        return format!("Hello Mike, how are you handling the situation with town?");
    }
    return format!("Hello, {name}, You are",);
}

#[get("/pizza")]
async fn get_pizza() -> HttpResponse {
    HttpResponse::Ok().body("Returned pizza peperoni!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("...Initializing web server ");

    println!("Server started on port 8080...");

    HttpServer::new(|| App::new().service(index).service(get_pizza).service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
