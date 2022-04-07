use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};

use futures::stream::TryStreamExt;
use mongodb::{bson::{doc, oid::ObjectId, DateTime, Document}, Database};
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
        .route("/create_projects", get(create_projects))
        .route("/get_projects", get(get_projects))
        .route("/update_projects", get(update_projects))
        .route("/delete_projects", get(delete_projects))
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

async fn create_projects() -> Result<impl IntoResponse, impl IntoResponse> {
    let db = get_dbinfo().await;
    let collection = db.collection::<Document>("projects");

    let docs = vec![
        doc! {
            "project_name": "test1",
            "project_owner_id": 1,
            "start_date": bson::DateTime::now().to_rfc3339_string(),
            "end_date": bson::DateTime::now().to_rfc3339_string(),
            "project_member_id": 1
        },
        doc! {
            "project_name": "test1",
            "project_owner_id": 1,
            "start_date": bson::DateTime::now().to_rfc3339_string(),
            "end_date": bson::DateTime::now().to_rfc3339_string(),
            "project_member_id": 1
        },
    ];

    let inserted = collection.insert_many(&docs, None).await;
    match inserted {
        Ok(_r) => Ok((StatusCode::CREATED, Json(docs))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
    }
}

async fn get_projects() -> impl IntoResponse {
    let db = get_dbinfo().await;
    let collection = db.collection::<Projects>("projects");

    let filter = doc! { "project_name": "test1" };
    let find_options = FindOptions::builder().sort(doc! { "_id": -1 }).build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();

    let mut vec: Vec<Projects> = Vec::new();
    while let Some(project) = cursor.try_next().await.unwrap() {
        vec.push(project);
    }
    (StatusCode::OK, Json(vec))
}

async fn update_projects() -> impl IntoResponse {
    let db = get_dbinfo().await;
    let collection = db.collection::<Projects>("projects");

    let filter = doc! { "project_name": "test1" };
    let update = doc!{"$set": {"project_name": "test111"}};
    let result = collection.update_many(filter, update, None).await.unwrap();
    
    (StatusCode::OK, Json(result))
}

async fn delete_projects() -> impl IntoResponse {
    let db = get_dbinfo().await;
    let collection = db.collection::<Projects>("projects");

    let filter = doc! { "project_name": "test1" };
    let result = collection.delete_many(filter, None).await.unwrap();
    
    (StatusCode::OK, Json(result))
}

async fn get_dbinfo() -> Database {
    let mut client_options = ClientOptions::parse(env::var("MONGODB_CONNECTION_STRING").unwrap())
        .await
        .unwrap();
    client_options.app_name = Some("HappyProject".to_string());
    let client = Client::with_options(client_options).unwrap();

    client.default_database().unwrap()
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
struct Projects {
    _id: ObjectId,
    project_name: String,
    project_owner_id: u64,
    start_date: String,
    end_date: String,
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
