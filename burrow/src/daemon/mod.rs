use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use std::net::SocketAddr;
use tokio::sync::mpsc::{channel, Receiver, Sender};

mod command;
mod instance;

use command::DaemonCommand;
use instance::Instance;

#[derive(Clone)]
struct SyncData {
    tx: Sender<DaemonCommand>,
}

pub async fn daemon_main() {
    tracing_subscriber::fmt::init();

    let (tx, rx) = channel(2);

    let sync_data = SyncData { tx };
    let mut inst = Instance::new(rx);

    let app = Router::new()
        .route("/", post(post_command))
        .with_state(sync_data);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::debug!("listening on {}", addr);

    let service = app.into_make_service();
    tokio::join!(inst.run(), async {
        axum::Server::bind(&addr).serve(service).await.unwrap()
    });
}

async fn post_command(
    State(state): State<SyncData>,
    Json(req): Json<DaemonCommand>,
) -> impl IntoResponse {
    state.tx.send(req).await.unwrap();
    StatusCode::OK
}
