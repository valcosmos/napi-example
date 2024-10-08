#![deny(clippy::all)]
mod algo;
mod matrix;

use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}
