use askama::Template;
use axum::{
    response::Html,
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[tokio::main]
async fn main() {
    // Create the router
    let app = Router::new()
        .route("/", get(show_index)) // Route for main HTML template at "/"
        .nest_service(
            "/static", // Serve static files under "/static"
            get_service(ServeDir::new("static")).handle_error(|error: std::io::Error| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", error),
                )
            }),
        );

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Serve the main template (index.html) at "/"
async fn show_index() -> Html<String> {
    let template = IndexTemplate;
    Html(template.render().unwrap())
}
