use crate::structs::data_document::DataDocument;
use axum::body::Body;
use axum::extract::State;
use axum::http::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetByCode {
    code: String,
}

#[derive(Deserialize)]
pub struct GenerateCode {
    data: String,
}

pub enum HandlerResponse {
    Status(StatusCode),
    DataDocument((StatusCode, Json<DataDocument>)),
}

impl IntoResponse for HandlerResponse {
    fn into_response(self) -> Response<Body> {
        match self {
            HandlerResponse::Status(status) => status.into_response(),
            HandlerResponse::DataDocument((status, data)) => (status, data).into_response(),
        }
    }
}

pub async fn get_data_by_code(
    State(client): State<Client>,
    Json(payload): Json<GetByCode>,
) -> HandlerResponse {
    if payload.code.len() != 7 {
        return HandlerResponse::Status(StatusCode::BAD_REQUEST);
    }

    let coll: Collection<DataDocument> = client.database("short_url").collection("codes");

    let result = coll
        .find_one(
            doc! {
                "code": payload.code
            },
            None,
        )
        .await;

    if let Err(e) = result {
        tracing::error!("Error: {}", e);
        HandlerResponse::Status(StatusCode::INTERNAL_SERVER_ERROR)
    } else if let Ok(Some(doc)) = result {
        HandlerResponse::DataDocument((StatusCode::OK, Json(doc)))
    } else {
        HandlerResponse::Status(StatusCode::NOT_FOUND)
    }
}

pub async fn generate_code(
    State(client): State<Client>,
    Json(payload): Json<GenerateCode>,
) -> HandlerResponse {
    let coll: Collection<DataDocument> = client.database("short_url").collection("codes");

    let code: &str = "1234567";

    let data: DataDocument = DataDocument {
        code: code.to_string(),
        data: payload.data,
        created_at: chrono::Utc::now(),
        updated_at: None,
    };

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        coll.insert_one(&data, None).await;

    if let Err(e) = result {
        tracing::error!("Error: {}", e);
        HandlerResponse::Status(StatusCode::INTERNAL_SERVER_ERROR)
    } else if result.is_ok() {
        return HandlerResponse::DataDocument((StatusCode::OK, Json(data)));
    } else {
        tracing::error!("Error inserting document into database.");
        return HandlerResponse::Status(StatusCode::INTERNAL_SERVER_ERROR);
    }
}
