use axum::{
    extract::State,
    response::{sse::Event, Sse},
};
use std::convert::Infallible;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct SseState {
    tx: broadcast::Sender<String>,
}

impl SseState {
    pub fn init() -> SseState {
        let (tx, _rx) = broadcast::channel::<String>(100);
        SseState { tx }
    }

    pub fn send(&self, msg: String) -> Result<usize, broadcast::error::SendError<String>> {
        self.tx.send(msg)
    }

    pub async fn handler(
        State(state): State<Self>,
    ) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
        let mut rx = state.tx.subscribe();

        let stream = async_stream::stream! {
            while let Ok(msg) = rx.recv().await {
                yield Ok(Event::default().data(msg));
            }
        };

        Sse::new(stream)
    }
}
