use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use utils::mongo_errors;

use crate::models::project;
use futures::stream::TryStreamExt;
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::{bson::doc, Client, Database};
use std::env;
use std::net::SocketAddr;

mod models {
    pub mod project;
}

mod utils {
    pub mod mongo_errors;
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/check", get(check))
        .route("/create_projects", post(create_projects))
        .route("/get_projects", get(get_projects))
        .route("/update_projects", get(update_projects))
        .route("/delete_projects", get(delete_projects))
        .route("/create_users", post(create_users));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn check() -> &'static str {
    "It's working."
}

async fn create_projects(Json(payload): Json<Vec<project::Projects>>) -> impl IntoResponse {
    let db = get_dbinfo().await;
    let collection = db.collection::<project::Projects>("projects");
    let result = collection.insert_many(payload, None).await;
    match result {
        Ok(r) => Ok((StatusCode::CREATED, Json(r))),
        Err(e) => Err(mongo_errors::mongo_errors(e)),
    }
}

async fn get_projects() -> impl IntoResponse {
    let db = get_dbinfo().await;
    let collection = db.collection::<project::Projects>("projects");

    let filter = doc! { "project_name": "test1" };
    let find_options = FindOptions::builder().sort(doc! { "_id": -1 }).build();
    let result = collection.find(filter, find_options).await;
    match result {
        Ok(mut r) => {
            let mut vec: Vec<project::Projects> = Vec::new();
            while let Some(project) = r.try_next().await.unwrap() {
                vec.push(project);
            }
            Ok((StatusCode::OK, Json(vec)))
        }
        Err(e) => Err(mongo_errors::mongo_errors(e)),
    }
}

async fn update_projects() -> impl IntoResponse {
    let db = get_dbinfo().await;
    let collection = db.collection::<project::Projects>("projects");

    let filter = doc! { "project_name": "test1" };
    let update = doc! {"$set": {"project_name": "test111"}};
    let result = collection.update_many(filter, update, None).await;
    match result {
        Ok(r) => Ok((StatusCode::OK, Json(r))),
        Err(e) => Err(mongo_errors::mongo_errors(e)),
    }
}

async fn delete_projects() -> impl IntoResponse {
    let db = get_dbinfo().await;
    let collection = db.collection::<project::Projects>("projects");

    let filter = doc! { "project_name": "test1" };
    let result = collection.delete_many(filter, None).await;
    match result {
        Ok(r) => Ok((StatusCode::CREATED, Json(r))),
        Err(e) => Err(mongo_errors::mongo_errors(e)),
    }
}

async fn get_dbinfo() -> Database {
    let mut client_options = ClientOptions::parse(env::var("MONGODB_CONNECTION_STRING").unwrap())
        .await
        .unwrap();
    client_options.app_name = Some("HappyProject".to_string());
    let client = Client::with_options(client_options).unwrap();
    client.default_database().unwrap()
}

async fn create_users(Json(payload): Json<Vec<project::Users>>) -> impl IntoResponse {
    let db = get_dbinfo().await;
    let collection = db.collection::<project::Users>("users");
    let inserted = collection.insert_many(payload, None).await;
    match inserted {
        Ok(r) => Ok((StatusCode::CREATED, Json(r))),
        Err(e) => Err(mongo_errors::mongo_errors(e)),
    }
}
