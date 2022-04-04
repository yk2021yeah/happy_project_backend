use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::prelude::*;
use futures::stream::TryStreamExt;
use mongodb::bson;
use mongodb::bson::{datetime, doc, oid::ObjectId, DateTime, Document};
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::result::Result;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/check", get(check))
        .route("/create_project", get(create_project))
        .route("/find_restaurants", get(get_restaurants))
        .route("/users", post(create_user));

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

async fn create_project() -> Result<impl IntoResponse, impl IntoResponse> {
    let connection = env::var("MONGODB_CONNECTION_STRING");
    println!("Connection String {}", connection.unwrap());
    let mut client_options = ClientOptions::parse(env::var("MONGODB_CONNECTION_STRING").unwrap())
        .await
        .unwrap();
    client_options.app_name = Some("HappyProject".to_string());
    let client = Client::with_options(client_options).unwrap();

    let db = client.default_database().unwrap();
    let collection = db.collection::<Document>("projects");

    let docs = vec![
        doc! {
            "project_name": "test1",
            "project_owner_id": "1",
            "start_date": bson::DateTime::now(),
            "end_date": bson::DateTime::now(),
            "project_member_id": "1"
        },
        doc! {
            "project_name": "test1",
            "project_owner_id": "1",
            "start_date": bson::DateTime::now(),
            "end_date": bson::DateTime::now(),
            "project_member_id": "1"
        },
    ];

    let inserted = collection.insert_many(&docs, None).await;
    match inserted {
        Ok(_r) => Ok((StatusCode::CREATED, Json(docs))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
    }
}

async fn get_restaurants() -> impl IntoResponse {
    let mut client_options = ClientOptions::parse(env::var("MONGODB_CONNECTION_STRING").unwrap())
        .await
        .unwrap();
    client_options.app_name = Some("HappyProject".to_string());
    let client = Client::with_options(client_options).unwrap();

    let db = client.default_database().unwrap();
    let collection = db.collection::<Restaurant>("restaurants");

    let filter = doc! { "cuisine": "American", "name": "YesNo Restaurant" };
    let find_options = FindOptions::builder().sort(doc! { "_id": -1 }).build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();

    let mut vec: Vec<Restaurant> = Vec::new();
    while let Some(restaurant) = cursor.try_next().await.unwrap() {
        vec.push(restaurant);
    }
    (StatusCode::OK, Json(vec))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[derive(Debug, Serialize, Deserialize)]
struct Restaurant {
    _id: ObjectId,
    name: String,
    cuisine: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Projects {
    _id: ObjectId,
    project_name: String,
    project_owner_id: u64,
    start_date: DateTime,
    end_date: DateTime,
    project_member_id: u64,
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
