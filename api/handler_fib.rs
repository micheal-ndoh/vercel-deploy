use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
mod fib;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
mod fib;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let n_param = req.uri().query().unwrap_or("");
    let params: HashMap<_, _> = url::form_urlencoded::parse(n_param.as_bytes())
        .into_owned()
        .collect();
    let n_str = params.get("n");

    let response = match n_str.and_then(|s| i64::from_str(s).ok()) {
        Some(n) if n >= 0 => {
            let result = fib::fibonacci(n);
            json!({
                "input": n,
                "fibonacci": result.to_string()
            })
        }
        _ => {
            json!({
                "error": "Please provide a valid non-negative integer as 'n' in the query string. Example: ?n=10"
            })
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(response.to_string().into())?)
}


pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let n_param = req.uri().query().unwrap_or("");
    let params: HashMap<_, _> = url::form_urlencoded::parse(n_param.as_bytes())
        .into_owned()
        .collect();
    let n_str = params.get("n");

    let response = match n_str.and_then(|s| i64::from_str(s).ok()) {
        Some(n) if n >= 0 => {
            let result = fib::fibonacci(n);
            json!({
                "input": n,
                "fibonacci": result.to_string()
            })
        }
        _ => {
            json!({
                "error": "Please provide a valid non-negative integer as 'n' in the query string. Example: ?n=10"
            })
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(response.to_string().into())?)
}
