use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
mod fib;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "你好，世界"
            })
            .to_string()
            .into(),
        )?)
}
