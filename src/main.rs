use actix_web::{get, web, App, HttpServer, Responder};

// Implement handlers for requests
#[get("/")]
async fn index() -> impl Responder {
    "Hello, index!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>, iden: web::Query<String>) -> impl Responder {
    format!("Hello, {name}, You are {iden}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("...Initializing web server ");

    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
