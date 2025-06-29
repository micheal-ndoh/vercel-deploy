use num_bigint::BigInt;
use num_traits::FromPrimitive;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

pub fn fibonacci(n: i64) -> BigInt {
    match n {
        0 => BigInt::from_i64(0).unwrap(),
        1 => BigInt::from_i64(1).unwrap(),
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
