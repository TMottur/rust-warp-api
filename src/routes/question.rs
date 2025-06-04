use std::collections::HashMap;
use warp::http::StatusCode;
use tracing::info;

use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, QuestionId, NewQuestion};
use handle_errors::Error;
use warp::{Rejection, Reply};
use types::pagination::Pagination;
use tracing::{event, instrument, Level};
use crate::types::pagination::Pagination;

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    event!(target: "practical_rust_book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
        
        info!(pagination = false);
        let res: Vec<Question> = match store
            .get_questions(pagination.limit, pagination.offset)
            .await {
                Ok(res) => res,
                Err(e) => {
                    return Err(warp::reject::custom(
                        Error::DatabaseQueryError(e)
                    ))
                },
            };
        Ok(warp::reply::json(&res))
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_question(question, id).await {
        Ok(res) => res,
        Err(e) => return 
            Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    };

    Ok(warp::reply::json(&res))
}

pub async fn delete_question(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }

    Ok(warp::reply::with_status(
        format!("Question {} deleted", id),
        StatusCode::OK)
    )
}

