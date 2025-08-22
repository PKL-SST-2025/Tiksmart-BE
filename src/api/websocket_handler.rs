use crate::AppState; // Import AppState
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
    Extension,
};
use dashmap::DashMap;
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::info;

// This type alias can now be removed as we'll pass the Arc<DashMap> directly.
type ProjectWsSenders = Arc<DashMap<i32, broadcast::Sender<String>>>;

/// WebSocket handler for project-specific updates.
/// GET /api/projects/:project_id/ws
pub async fn project_ws_handler(
    ws: WebSocketUpgrade,
    Path(project_id): Path<i32>,
    Extension(user_id): Extension<i32>,
    State(app_state): State<AppState>, // FIX: Extract AppState
) -> impl IntoResponse {
    // 1. Authorize user access to the project


    info!("WebSocket: User {} attempting to connect to project {}", user_id, project_id);

    // 2. Upgrade the HTTP connection to a WebSocket
    ws.on_upgrade(move |socket| {
        handle_project_socket(
            socket,
            project_id,
            user_id,
            app_state.project_ws_senders.clone(), // Clone the Arc and pass it
        )
    })
}

async fn handle_project_socket(
    socket: WebSocket,
    project_id: i32,
    user_id: i32,
    project_ws_senders: Arc<DashMap<i32, broadcast::Sender<String>>>, // Accept the cloned Arc
) {
    info!("WebSocket: User {} connected to project {}", user_id, project_id);

    let (mut sender_ws, mut receiver_ws) = socket.split();

    let sender_broadcast = project_ws_senders
        .entry(project_id)
        .or_insert_with(|| {
            info!("WebSocket: Created new broadcast channel for project {}", project_id);
            broadcast::channel::<String>(16).0
        })
        .clone();

    let mut receiver_broadcast = sender_broadcast.subscribe();

    // Spawn a task to send messages from the broadcast channel to the WebSocket client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = receiver_broadcast.recv().await {
            if sender_ws.send(Message::Text(msg)).await.is_err() {
                info!("WebSocket: Failed to send message to user {} for project {}. Disconnecting.", user_id, project_id);
                break;
            }
        }
    });

    // Spawn a task to receive messages from the WebSocket client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver_ws.next().await {
            info!("WebSocket: Received message from user {} for project {}: {}", user_id, project_id, text);
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    info!("WebSocket: User {} disconnected from project {}", user_id, project_id);
}