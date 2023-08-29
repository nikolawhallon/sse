use axum::{
    extract,
    response::sse::{Event, KeepAlive, Sse},
    routing::post,
    Router,
};
//use futures_util::stream::self;
use futures_util::stream::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
//use std::time::Duration;
use tokio_stream::StreamExt as _;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyPayload {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/sse", post(sse_handler));

    axum_server::bind("0.0.0.0:5000".to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn sse_handler(
    extract::Json(payload): extract::Json<MyPayload>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    /*
    let stream = stream::repeat_with(move || Event::default().data(payload.message.clone()))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
    */

    let (s, r) = async_channel::unbounded();
    s.send(payload.message.clone()).await.unwrap();
    s.send(payload.message.clone()).await.unwrap();
    s.send(payload.message.clone()).await.unwrap();
    s.send(payload.message).await.unwrap();

    Sse::new(r.map(|x| Ok(Event::default().data(x)))).keep_alive(KeepAlive::default())
}
