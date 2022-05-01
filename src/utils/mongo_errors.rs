use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::error::{Error, ErrorKind};

pub fn mongo_errors(e: Error) -> Result<(), impl IntoResponse> {
    // print error detail as Server log
    println!("{}", e.to_string());
    // Return http status and quick message to the client
    match *e.kind {
        ErrorKind::InvalidArgument { message, .. } => Err((StatusCode::BAD_REQUEST, Json(message))),
        ErrorKind::Authentication { message, .. } => Err((StatusCode::UNAUTHORIZED, Json(message))),
        ErrorKind::BsonDeserialization(_) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
        ErrorKind::BsonSerialization(_) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
        ErrorKind::BulkWrite(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
        ErrorKind::Command(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
        ErrorKind::DnsResolve { message, .. } => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
        }
        ErrorKind::Internal { message, .. } => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
        }
        ErrorKind::Io(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
        ErrorKind::ConnectionPoolCleared { message, .. } => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
        }
        ErrorKind::InvalidResponse { message, .. } => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
        }
        ErrorKind::ServerSelection { message, .. } => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
        }
        ErrorKind::SessionsNotSupported => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
        ErrorKind::InvalidTlsConfig { message, .. } => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
        }
        ErrorKind::Write(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
        ErrorKind::Transaction { message, .. } => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
        }
        ErrorKind::IncompatibleServer { message, .. } => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
        }
        _ => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
    }
}
