use axum::{
    routing::{get, put},
    Router,
};

mod handlers;

// selecting the single-threaded mode, as I want to run this code in an SGX enclave, which also starts of single-threaded. SGX can be multi-threaded but that is not the default
#[tokio::main(flavor="current_thread")]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("{vaultBaseUrl}/secrets/{secret-name}/{secret-version}", put(handlers::set_secret))
        .route("{vaultBaseUrl}/secrets/{secret-name}/{secret-version}", get(handlers::get_secret))
        .route("/", get(|| async { "Hello, World!\n...from the keystore" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}