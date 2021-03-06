use actix_web::{
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use mongodb::{Client, options::ClientOptions};

use futures::stream::StreamExt;
use mongodb::{
    bson::{doc},
    options::FindOptions,
};


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
    owner: String
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
    let mut client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await.expect("Some error message");
    client_options.app_name = Some("athene-list_task".to_string());
    let client = Client::with_options(client_options).expect("Some error message");
    let db = client.database("athene");
    
    let collection = db.collection("tasks");
    
    // Query the documents in the collection with a filter and an option.
    //let filter = doc! { "owner": "George Orwell" };
    let filter = doc! { };
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = collection.find(filter, find_options).await.expect("Some error message");

    // Iterate over the results of the cursor.
    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
               results.push(document);
            }
            Err(_) => {},
        }
    }
    Ok(HttpResponse::Ok().json(results))
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
    //let bind = "0.0.0.0:8080";
    let bind = "127.0.0.1:8080";
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