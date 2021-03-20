use actix_web::{
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    id: String,
    date: String,
    creator: String,
    asigned: String,
    description: String,
    text: String,
    labels: Vec<String>,
    create_date: String,
    state: TaskState
}


#[derive(Debug, Serialize, Deserialize)]
struct TaskList {
    title: String,
    id: String,
    date: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum TaskState {
    Created,
    Started,
    Finished,
    Closed
}

async fn create_task(req: HttpRequest) -> impl Responder {
    let task_id = req.match_info().get("task_id").unwrap_or("World");
    format!("Hello {}!", &task_id)
}

async fn read_task(task_id: web::Path<String>) -> Result<HttpResponse> {
    
    let result = Task { 
        title: "t".to_string(),
        id: task_id.to_owned(), 
        date:"2020-01-02".to_string(), 
        creator:"peter".to_string(), 
        asigned: "peter".to_string(), 
        description:"hannelore".to_string(), 
        text:"hannelore beschreibt peter".to_string(), 
        labels: vec!["".to_string()], 
        create_date: "".to_string(), 
        state: TaskState::Created
    };

    Ok(HttpResponse::Ok().json(result))
}

async fn list_task() -> Result<HttpResponse> {
    
    let result = vec![TaskList { 
        title: "t".to_string(),
        id: "task_id".to_owned(), 
        date:"2020-01-01".to_string(), 
    }, TaskList { 
        title: "t2".to_string(),
        id: "task_id2".to_owned(), 
        date:"2020-01-02".to_string(), 
    }];

    Ok(HttpResponse::Ok().json(result))
}



async fn update_task(req: HttpRequest) -> impl Responder {
    println!("update_task");
    let task_id = req.match_info().get("task_id").unwrap_or("World");
    format!("Hello {}!", &task_id)
}

async fn delete_task(req: HttpRequest) -> impl Responder {
    println!("delete_task");
    let task_id = req.match_info().get("task_id").unwrap_or("World");
    format!("Hello {}!", &task_id)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let bind = "0.0.0.0:8080";
    println!("Athene Task Module starting at {}", bind);
    HttpServer::new(|| {
        App::new()
            // middleware
            .wrap(middleware::Logger::default())

            // config
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/")
                .route(web::get().to(list_task)))
            // routing
            .service(web::resource("/{task_id}")
                .route(web::post().to(create_task))
                .route(web::get().to(read_task))
                .route(web::put().to(update_task))
                .route(web::delete().to(delete_task)))
    })
    .bind(bind)?
    .run()
    .await
    
}