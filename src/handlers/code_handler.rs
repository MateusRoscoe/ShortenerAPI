use crate::helpers::common::to_base62;
use crate::structs::data_document::DataDocument;
use axum::body::Body;
use axum::extract::Query;
use axum::extract::State;
use axum::http::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::bson::doc;
use mongodb::{Collection, Database};
use serde::Deserialize;
use std::sync::atomic::{AtomicU64, Ordering};

static COLLECTION: &str = "codes";
static COUNTER: AtomicU64 = AtomicU64::new(1);

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
    State(database): State<Database>,
    Query(payload): Query<GetByCode>,
) -> HandlerResponse {
    let coll: Collection<DataDocument> = database.collection(COLLECTION);

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
    State(database): State<Database>,
    Json(payload): Json<GenerateCode>,
) -> HandlerResponse {
    let coll: Collection<DataDocument> = database.collection(COLLECTION);

    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    let code: String = to_base62(counter);

    let data: DataDocument = DataDocument {
        code: code.to_string(),
        data: payload.data,
        created_at: chrono::Utc::now(),
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
