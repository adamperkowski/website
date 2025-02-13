use axum::{
    extract::State,
    response::{sse::Event, Sse},
};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct SseState {
    pub tx: broadcast::Sender<String>,
}

/// Initialize the SSE state.
pub fn init() -> SseState {
    let (tx, _rx) = broadcast::channel::<String>(100);
    SseState { tx }
}

/// The event handler yuuuup...
pub async fn handler(
    State(state): State<Arc<SseState>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let mut rx = state.tx.subscribe();

    let stream = async_stream::stream! {
        while let Ok(msg) = rx.recv().await {
            yield Ok(Event::default().data(msg));
        }
    };

    Sse::new(stream)
}
