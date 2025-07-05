// The 'use' keyword is similar to C#'s 'using' directive. It brings items
// (modules, functions, types) into scope so you can use them without
// specifying their full path.

// Axum specific imports:
// Router: The core type for defining your web application's routes.
// routing::{get, post}: Functions to define HTTP GET and POST routes.
// http::StatusCode: For HTTP status codes (e.g., 200 OK, 404 Not Found).
// response::{Html, Json}: Helper types for returning HTML and JSON responses.
use axum::{
    routing::{get, post}, // We'll primarily use GET for SSR, static files, and some API, POST for API actions.
    http::StatusCode,
    response::{Html, Json},
    Router,
};

// Tokio specific imports:
// net::TcpListener: For creating a TCP listener, similar to C#'s TcpListener.
use tokio::net::TcpListener;

// Standard library imports:
use std::net::SocketAddr; // For specifying IP address and port (e.g., 127.0.0.1:3000).
use std::path::PathBuf;   // For working with file system paths.
use std::env;             // For environment variables, specifically to find the executable path.

// Tower-http specific imports:
// services::ServeDir: A service for serving static files from a directory.
use tower_http::services::ServeDir;

// Tracing imports for logging:
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use chrono; // Brings the chrono crate into scope

use html_pages::home_page;
use pages_components::ButtonComponent;

// This attribute is crucial for async Rust.
// #[tokio::main] is a macro that transforms the `main` function into an asynchronous entry point.
// It sets up the Tokio runtime, allowing you to use `async` and `await` keywords.
// Think of it like `Task.Run` or configuring an async context for your main method in C#.
#[tokio::main]
async fn main() {
    // 1. Initialize Tracing (Logging Setup)
    // This sets up a subscriber that will print log messages to your console.
    // It's very useful for debugging, similar to `Console.WriteLine` or a logging framework in C#.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,my_ssr_server=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

      println!("Server application starting...");

    // 2. Determine Static Files Directory
    // Your requirement is to serve static files from a 'static' directory
    // placed at the same directory as the .exe.
    // This code dynamically finds the executable's path and appends "static".
    // It's similar to `AppDomain.CurrentDomain.BaseDirectory` in C#.
    let current_exe_path = env::current_exe()
        .expect("Failed to get current executable path");

    // Get the parent directory of the executable (where the .exe itself resides).
    let current_dir = current_exe_path.parent()
        .expect("Failed to get parent directory of executable");

    // Construct the path to the 'static' directory.
    let static_files_path = current_dir.join("static");

    // Log the path to confirm it's correct.
    tracing::info!("Serving static files from: {:?}", static_files_path);

    // 3. Define the Router (HTTP Server Configuration)
    // `Router::new()` creates a new, empty router.
    // `.route("/", get(index_handler))` adds a route for the root path ("/").
    //   - `get(index_handler)` means this route will only respond to HTTP GET requests,
    //     and when a GET request comes in for "/", it will call the `index_handler` function.
    // `.route("/api/data", get(api_data_handler))` sets up an API endpoint.
    // `.fallback(get(static_file_handler))` used to be a way to serve static files.
    //    However, `tower_http::services::ServeDir` is now the recommended and more robust way.
    // `.nest_service("/static", ServeDir::new(static_files_path.clone()))` is the key for static files.
    //    It tells Axum to serve any requests starting with `/static` (e.g., `/static/style.css`)
    //    from the `static_files_path` directory on the file system.
    //    The `clone()` is needed because `static_files_path` is moved into `ServeDir::new`.
    //    Alternatively, we can set up `ServeDir` as a fallback to serve anything that isn't a defined route.
    //    This means if `index_handler` or `api_data_handler` don't match, Axum will try to find the file
    //    in the static directory. This is often more convenient for general static file serving.

    // Important: We'll use `ServeDir` as a final fallback for requests that don't match other routes.
    // This means if a request for `/my-image.png` comes in, and no other route handles it,
    // `ServeDir` will look for `my-image.png` inside the `static_files_path` directory.
    let app = Router::new()
        // Website route (text/html)
        .route("/", get(index_handler))
        // API route (text/json)
        .route("/api/data", get(api_data_handler))
        // Static files (catch-all for anything not explicitly routed)
        // This makes it so that if a request doesn't match "/" or "/api/data",
        // it tries to find the file directly in the `static_files_path` directory.
        // For example, a request to `/style.css` will look for `static/style.css`.
        // .nest_service("/static", ServeDir::new(static_files_path));
        .fallback_service(ServeDir::new(static_files_path));


    // 4. Start the Server
    // Define the address to listen on. `0.0.0.0:3000` means listen on all available
    // network interfaces on port 3000. For local development, `127.0.0.1:3000` (localhost) is also common.
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);

    // Create a TCP listener. This is an async operation.
    // `.await` pauses the execution of this `async` function until the `bind` operation completes.
    // It's similar to `await`ing a `Task` in C#.
    let listener = TcpListener::bind(addr).await
        .expect("Failed to bind TCP listener"); // Handle potential errors, like port already in use.

    // Start serving the application. `axum::serve` is the entry point for Axum's server.
    // It takes the TCP listener and the router, and starts handling incoming connections.
    // This call is also `async` and will run indefinitely until the server is stopped.
    axum::serve(listener, app)
        .await
        .expect("Failed to start server"); // Handle potential errors during server startup.
}

// Handler function for the root route ("/").
// This function must be `async` because Axum handlers are asynchronous.
// It returns `Html<String>`, indicating that the response content type will be `text/html`.
async fn index_handler() -> Html<String> {
    tracing::info!("Received request for /");

    let page_level_data = vec![
        String::from("<p>This is dynamic page-level data.</p>"),
        String::from("<p>More data here!</p>"),
    ];

    let css_files_markup = vec![
        String::from(r#"<link rel="stylesheet" href="/static/global.css">"#),
        // home_page.css will be included by home_page.rs
    ];

    let components_markup = vec![
        String::from(r#"<div class="some-other-component">This is a generic component placeholder!</div>"#),
        // The button component will be added by home_page.rs
    ];

    let js_files_markup = vec![
        String::from(r#"<script src="/static/global.js"></script>"#),
        // home_page.js will be included by home_page.rs
    ];

    // Call the print_page function from your new module
    let full_html = html_pages::home_page::print_page( // Use full path here for clarity
        &page_level_data,
        &css_files_markup,
        &components_markup,
        &js_files_markup,
    );

    Html(full_html)
}

// Handler function for the API route ("/api/data").
// It returns `Json<serde_json::Value>`, which means the response content type will be `application/json`.
// `serde_json::Value` is a generic JSON value type, similar to `JObject` or `dynamic` in C# for JSON.
async fn api_data_handler() -> Json<serde_json::Value> {
    tracing::info!("Received request for /api/data"); // Log the request.
    // Create a JSON object using `serde_json::json! macro`.
    // This is a convenient way to construct JSON values.
    Json(serde_json::json!({
        "message": "Hello from the Rust API!",
        "timestamp": chrono::Utc::now().to_rfc3339(), // Example: add a timestamp
        "data": {
            "key1": "value1",
            "key2": 123
        }
    }))
}

// Notes on `Result` and Error Handling (Rust vs C#):
// In Rust, error handling primarily uses the `Result` enum (`Ok(T)` for success, `Err(E)` for error).
// Functions that can fail often return `Result<T, E>`.
// The `?` operator (seen in `TcpListener::bind(addr).await?`) is a concise way to propagate errors.
// If the `Result` is `Err`, it immediately returns that error from the current function.
// If it's `Ok`, it unwraps the `Ok` value and continues.
// This is Rust's way of dealing with "exceptions" without throwing them, providing
// compile-time guarantees that you've considered potential failures.
// In our `main` function, we use `.expect("message")` which is a shortcut to
// unwrap the `Result` but will panic (crash) if it's an `Err`. For production code,
// you'd typically handle `Result` more gracefully, but for initial learning, `.expect` is fine.