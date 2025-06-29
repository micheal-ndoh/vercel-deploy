use bigInt::BigInt;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

pub fn fibonacci(n: i64) -> BigInt {
    match n {
        0 => BigInt::from(0),
        1 => BigInt::from(1),
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
