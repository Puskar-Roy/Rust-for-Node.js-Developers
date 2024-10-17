use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[derive(Serialize)]
struct Person {
    name: String,
    age: String,
}

async fn get_json_data() -> HttpResponse {
    let response = Person {
        name: "Good!".to_string(),
        age: "21".to_string(),
    };

    HttpResponse::Ok().json(response)
}

async fn post_json(info: web::Json<Info>) -> HttpResponse {
    HttpResponse::Ok().json(format!("Received name: {}", info.name))
}

async fn get_user(path: web::Path<(u32,)>) -> HttpResponse {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("User ID is {}", user_id.0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at: http://localhost:3000");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())  
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello World!") }))
            .route("/users", web::get().to(get_json_data))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(post_json))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
