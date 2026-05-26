use std::{convert::Infallible, time::Duration};

use axum::{
    Router,
    extract::{
        Multipart, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::{
        Html, IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{get, post},
};
use futures::{
    StreamExt as FuturesStreamExt,
    stream::{self, Stream},
};
use tokio_stream::StreamExt as TokioStreamExt;

async fn home() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <title>Module 10: Advanced Features</title>
    <style>
        body {
            font-family: system-ui, sans-serif;
            max-width: 800px;
            margin: 40px auto;
            padding: 20px;
        }

        .demo {
            background: #f5f5f5;
            padding: 20px;
            border-radius: 8px;
            margin-bottom: 20px;
        }

        input, button {
            padding: 8px 12px;
            font-size: 16px;
        }

        #ws-output,
        #sse-output {
            margin-top: 12px;
            padding: 12px;
            background: white;
            border: 1px solid #ddd;
            min-height: 80px;
        }
    </style>
</head>
<body>
    <h1>Module 10: Advanced Features</h1>

    <div class="demo">
        <h2>WebSocket Echo</h2>

        <input id="ws-input" type="text" placeholder="Digite uma mensagem">
        <button onclick="sendMessage()">Enviar</button>

        <div id="ws-output"></div>
    </div>
    <div class="demo">
        <h2>Server-Sent Events</h2>

        <button onclick="startSse()">Iniciar SSE</button>
        <button onclick="stopSse()">Parar SSE</button>

        <div id="sse-output"></div>
    </div>

    <script>
        const output = document.getElementById("ws-output");
        const input = document.getElementById("ws-input");

        const sseOutput = document.getElementById("sse-output");
        let eventSource = null;

        const socket = new WebSocket("ws://localhost:8000/ws");

        socket.addEventListener("open", () => {
            output.innerHTML += "<p>Conectado ao WebSocket.</p>";
        });

        socket.addEventListener("message", (event) => {
            output.innerHTML += `<p>${event.data}</p>`;
        });

        socket.addEventListener("close", () => {
            output.innerHTML += "<p>Conexão fechada.</p>";
        });

        function sendMessage() {
            const message = input.value;

            if (!message) {
                return;
            }

            socket.send(message);
            input.value = "";
        }

        function startSse() {
            if (eventSource) {
                return;
            }

            eventSource = new EventSource("/sse");

            eventSource.addEventListener("message", (event) => {
                sseOutput.innerHTML = `<p>${event.data}</p>`;
            });

            eventSource.addEventListener("open", () => {
                sseOutput.innerHTML = "<p>SSE conectado.</p>";
            });

            eventSource.addEventListener("error", () => {
                sseOutput.innerHTML += "<p>Erro ou conexão SSE encerrada.</p>";
            });
        }

        function stopSse() {
            if (!eventSource) {
                return;
            }

            eventSource.close();
            eventSource = null;

            sseOutput.innerHTML += "<p>SSE parado.</p>";
        }
    </script>
</body>
</html>
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

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        let now = std::time::SystemTime::now();

        Event::default().data(format!("Server time: {now:?}"))
    });

    let stream = FuturesStreamExt::map(stream, Ok);

    let stream = TokioStreamExt::throttle(stream, Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    let mut files = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unknown").to_string();
        let file_name = field.file_name().unwrap_or("unnamed").to_string();

        let data = field.bytes().await.unwrap();

        files.push(format!("{name} ({file_name}): {} bytes", data.len()));
    }

    if files.is_empty() {
        "No files uploaded".to_string()
    } else {
        format!("Uploaded: {}", files.join(", "))
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/ws", get(ws_handler))
        .route("/sse", get(sse_handler))
        .route("/upload", post(upload));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("🚀 Module 10: Advanced Features");
    println!("Server: http://localhost:8000");

    axum::serve(listener, app).await.unwrap();
}

/*
Teste com:

cargo r10

curl -i -w '\n\n' http://localhost:8000/

Teste WebSocket no Bruno:

ws://localhost:8000/ws

Envie:
Oi Axum

Resposta esperada:
Echo: Oi Axum

Teste SSE:

curl -i -N http://localhost:8000/sse

Teste upload:

echo "Hello upload" > test-upload.txt

curl -i -w '\n\n' -X POST http://localhost:8000/upload \
  -F 'file=@test-upload.txt'
*/
