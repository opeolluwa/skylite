use reqwest::Method;

use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use axum::extract::DefaultBodyLimit;
use local_ip_address::local_ip;

use lazy_static::lazy_static;
mod router;
mod routes;

lazy_static! {
    /**
 * the lazy static crate allow the lazy evaluation of constants thus, one can bypass the impossible dynamic bindings of constants
 *
 *
 * Herein the server port made globally available, this allow for ease of sharing same with file upload directory
 */
    pub static ref SERVER_PORT: u16 = 2105;
    pub static ref UPLOAD_DIRECTORY: std::string::String = String::from("share");
}

/**
 * @function core_server
 * the application core responsible for handling file upload to client
 *  machine and file download to the host machine
 */
pub async fn run() {
    // initialize database
    // initialize tracing subscriber
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "share=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init(); // allow debugging in development set up

    // define cors scope as any
    // change this later to only allow get and post http verbs
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST]) // restrict methods
        .allow_origin(Any); // TODO: restrict this in the future to only sendfile proxy server for example http://sendfile/dhsdo

    // define file limit layer as 10GB
    // see information here <https://docs.rs/axum/0.6.2/axum/extract/struct.DefaultBodyLimit.html#%E2%80%A6>
    let file_size_limit = 10 * 1024 * 1024 * 1024;
    let file_limit = DefaultBodyLimit::max(file_size_limit);

    //  run the https server on localhost then feed off the connection using the wifi gateway, the same way Vite/Vue CLI would do the core server
    // this is currently achieved by binding the server to the device default ip address
    let my_local_ip = local_ip().unwrap();
    let ip_address = format!("{:?}:{:?}", my_local_ip, *SERVER_PORT as u64);
    let ip_address = ip_address
        .parse::<std::net::SocketAddr>()
        .expect("invalid socket address");

    println!("server running on http://{}", &ip_address.to_string());

    // build our application with the required routes
    let app = router::app()
        .layer(file_limit)
        .layer(cors_layer)
        .layer(tower_http::trace::TraceLayer::new_for_http());
    // .fallback(handle_404);

    // add a fallback service for handling routes to unknown paths
    // let app = app.fallback(handle_404);

    // run the server
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
