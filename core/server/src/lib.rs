use std::net::IpAddr;
use std::net::Ipv4Addr;

use axum::http::HeaderValue;
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
 * the lazy static crate allow the lazy evaluation of constants thus, one can bypass the impossible dynamic bindings of constants and static variables
 *
 *
 * Herein the server port made globally available, this allow for ease of sharing same with file upload directory
 *
 */
    pub static ref SERVER_PORT: u16 = 2105;
    // the directory the files would be uploaded to
    pub static ref UPLOAD_DIRECTORY: std::string::String = String::from("skylite");

    /** // the server address
    * run the web server on the device Ip address,
    *  if the address is not found, fallback to 0.0.0.0:2105
    */
    pub static ref SERVER_ADDRESS: std::string::String = {

    let default_ip_address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let local_ip = local_ip().unwrap_or(default_ip_address);
     format!("{:?}:{:?}", local_ip, *SERVER_PORT as u64)
    };
}

/**
 * @function core_server
 * the application core responsible for handling file upload to client
 *  machine and file download to the host machine
 */
pub async fn run() {
    // initialize tracing subscriber
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "share=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init(); // allow debugging in development set up

    // define cors scope as any
    let allowed_origin = SERVER_ADDRESS.parse::<HeaderValue>().unwrap();
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST]) // restrict methods
        // restrict the service to only devices on the network
        .allow_origin(allowed_origin);

    // define file limit layer as 10GB
    // see information here <https://docs.rs/axum/0.6.2/axum/extract/struct.DefaultBodyLimit.html#%E2%80%A6>
    let file_size_limit = 10 * 1024 * 1024 * 1024;
    let file_limit = DefaultBodyLimit::max(file_size_limit);

    let ip_address = SERVER_ADDRESS
        .parse::<std::net::SocketAddr>()
        .expect("invalid socket address");

    println!("server running on http://{}", &ip_address.to_string());

    // build our application with the required routes
    let app = router::app()
        /*   .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true)) */
        .layer(file_limit)
        .layer(cors_layer)
        .layer(tower_http::trace::TraceLayer::new_for_http());
    // .fallback(handle_404);

    // run the server
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
