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

        #ws-output {
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

    <script>
        const output = document.getElementById("ws-output");
        const input = document.getElementById("ws-input");

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
