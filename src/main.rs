// Search modules in api, models, and repository folders

mod api; 
mod models;
mod repository;

use api::user_api::{create_course, delete_course, get_all_courses, get_course, update_course}; //import the handler here
use repository::mongodb_repo::MongoRepo;

use actix_web::{web::Data, get, App, HttpResponse, HttpServer, Responder};

use chrono::prelude::*;

// Main endpoint

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[{}] Starting server", Local::now());
    // Init repository
    let db = MongoRepo::init().await;
    // Init data
    let db_data = Data::new(db);
    // New server with HTTP protocol
    // Move ownership into the closure
    println!("[{}] Server listening at port 8080", Local::now());
    println!("Endpoints:");
    println!("POST /course");
    println!("BODY: code, name and credits");
    println!("GET /course/{{id}}");
    println!("PUT /course/{{id}}");
    println!("BODY: code, name and credits");
    println!("DELETE /course/{{id}}");
    println!("GET /courses");

    HttpServer::new(move || {
        // Create app with services (data, and CRUD oprations)
        App::new()
            .app_data(db_data.clone())
            .service(create_course)
            .service(get_course)
            .service(update_course) //add this
            .service(delete_course) //add this
            .service(get_all_courses)//add this
    })
    // Bind at IP and TCP protocols
    .bind(("127.0.0.1", 8080))?
    // Run an hold listening
    .run()
    .await
}