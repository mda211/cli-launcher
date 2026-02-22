use axum::{Router, extract::Query, routing::get};
use serde::Deserialize;
use tokio::sync::mpsc;

#[derive(Deserialize, Clone, Debug)]
pub struct CallbackParams {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

pub async fn start_auth_server() -> Result<CallbackParams, Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";

    let (tx, mut rx) = mpsc::channel(1);

    tokio::spawn(async move {
        let app = Router::new()
            .route("/auth/callback", get(callback_handler))
            .with_state(tx);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    if let Some(params) = rx.recv().await {
        Ok(params)
    } else {
        Err("Failed to receive callback".into())
    }
}

async fn callback_handler(
    Query(params): Query<CallbackParams>,
    axum::extract::State(tx): axum::extract::State<mpsc::Sender<CallbackParams>>,
) -> String {
    if let Some(error) = &params.error {
        let msg = format!("Authentication error: {}", error);
        let _ = tx
            .send(CallbackParams {
                code: None,
                state: None,
                error: Some(error.clone()),
            })
            .await;
        return msg;
    }

    if params.code.is_some() && params.state.is_some() {
        let _ = tx.send(params).await;
        "Login successful! You can close this window.".to_string()
    } else {
        "Error: Missing authorization code or state".to_string()
    }
}
