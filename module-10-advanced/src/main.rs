use axum::{
    Router,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::{Html, IntoResponse},
    routing::get,
};

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

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(result) = socket.recv().await {
        let Ok(message) = result else {
            break;
        };

        if let Message::Text(text) = message {
            let response = format!("Echo: {text}");

            if socket.send(Message::Text(response.into())).await.is_err() {
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/ws", get(ws_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("🚀 Module 10: Advanced Features");
    println!("Server: http://localhost:8000");

    axum::serve(listener, app).await.unwrap();
}

/*
Teste com:

cargo r10

curl -i -w '\n\n' http://localhost:8000/

Teste WebSocket no navegador:

1. Acesse:
   http://localhost:8000/

2. Abra o console do navegador e rode:

const ws = new WebSocket("ws://localhost:8000/ws");

ws.onmessage = (event) => console.log(event.data);

ws.onopen = () => ws.send("Oi Axum");
*/
