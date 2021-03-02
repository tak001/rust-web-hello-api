use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: Option<u32>,
    content: String,
    done: bool,
}

#[get("/todos/{id}")]
async fn get_todo(web::Path(id): web::Path<u32>) -> impl Responder {
    println!("get_todo");
    let id_option: Option<u32> = Some(id);
    HttpResponse::Ok().json(Todo {
        id: id_option,
        content: String::from("やること"),
        done: false,
    })
}

#[post("/todos")]
async fn post_todo(todo: web::Json<Todo>) -> impl Responder {
    println!("post_todo");
    println!("{:?}", todo);
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_todo).service(post_todo))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}