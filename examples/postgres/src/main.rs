use axum::{routing::get, Router};
use axum_session::{SessionConfig, SessionLayer, SessionPgSession, SessionPgSessionStore};
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

#[tokio::main]
async fn main() {
    let poll = connect_to_database().await;

    //This Defaults as normal Cookies.
    //To enable Private cookies for integrity, and authenticity please check the next Example.
    let session_config = SessionConfig::default().with_table_name("sessions_table");

    let session_store = SessionPgSessionStore::new(Some(poll.clone().into()), session_config)
        .await
        .unwrap();

    //Create the Database table for storing our Session Data.
    session_store.initiate().await.unwrap();

    // build our application with some routes
    let app = Router::new()
        .route("/greet", get(greet))
        .layer(SessionLayer::new(session_store));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn greet(session: SessionPgSession) -> String {
    let mut count: usize = session.get("count").unwrap_or(0);

    count += 1;
    session.set("count", count);

    count.to_string()
}

async fn connect_to_database() -> PgPool {
    let mut connect_opts = PgConnectOptions::new();
    connect_opts = connect_opts
        .database("test")
        .username("test")
        .password("password")
        .host("127.0.0.1")
        .port(5432);

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
        .unwrap()
}
