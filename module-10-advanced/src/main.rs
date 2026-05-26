use axum::{response::Html, routing::get, Router};

async fn home() -> Html<&'static str> {
    Html(
        r#"
        <h1>Module 10: Advanced Features</h1>

        <ul>
            <li>WebSocket</li>
            <li>Server-Sent Events</li>
            <li>File Upload</li>
            <li>Static Files</li>
        </ul>
        "#,
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();

    println!("🚀 Module 10: Advanced Features");
    println!("Server: http://localhost:8000");

    axum::serve(listener, app).await.unwrap();
}

/*
Teste com:

cargo r10

curl -i -w '\n\n' http://localhost:8000/
*/
