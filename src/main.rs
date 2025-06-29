#![warn(clippy::all)]

use config::Config;
use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
use clap::Parser;
use std::env;
use std::env::args;

mod profanity;
mod routes;
mod store;
mod types;

#[derive(Parser, Debug, Default, serde::Deserialize, PartialEq)]
struct Args {
    log_level: String,
    /// Which PORT the server is listening to
    #[clap(short, long, default_value = "8080")]
    port: u16,
    /// Database user
    #[clap(long, default_value = "user")]
    db_user: String,
    /// URL for the postgres database
    database_host: String,
    /// PORT number for the database connection
    database_port: u16,
    /// Database name
    database_name: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    
    let config = Config::builder()
        .add_source(config::File::with_name("setup"))
        .build()
        .unwrap();

    let config = config
        .try_deserialize::<Args>()
        .unwrap();

    let db_user = env::var("POSTGRES_USER")
        .unwrap_or(config.db_user.to_owned());
    let db_password = env::var("POSTGRES_PASSWORD").unwrap();
    let db_host = env::var("POSTGRES_HOST")
        .unwrap_or(config.database_host.to_owned());
    let db_port = env::var("POSTGRES_PORT")
        .unwrap_or(config.database_port.to_string());
    let db_name = env::var("POSTGRES_DB")
        .unwrap_or(config.database_name.to_owned());

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rust_web={},warp={}",
            config.log_level, config.log_level, config.log_level
        )
    });

    let store =
        store::Store::new(&format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        )).await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot migrate DB");

    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::PUT,
            Method::DELETE,
            Method::GET,
            Method::POST,
        ]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    let routes = get_questions
        .or(update_question)
        .or(add_question)
        .or(delete_question)
        .or(add_answer)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    tracing::info!("Q&A service build ID {:?}", std::env::var("RUST_WEB_DEV_VERSION"));

    warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;

    Ok(())

}