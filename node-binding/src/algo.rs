use algo::{Algo, AlgoType};
use napi_derive::napi;

#[napi(js_name = "Algo")]
pub struct JsAlgo {
  inner: Algo,
}

#[napi]
impl JsAlgo {
  #[napi(constructor)]
  pub fn new(name: String) -> Self {
    let algo = match name.as_str() {
      "blake3" => Algo::new(AlgoType::Blake3),
      _ => Algo::new(AlgoType::Default),
    };

    Self { inner: algo }
  }

  #[napi]
  pub fn hash(&self, v: String) -> String {
    self.inner.hash(v)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hash_should_work() {
    let algo = Algo::new(AlgoType::Blake3);

    assert_eq!(
      algo.hash("hello".to_string()),
      "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f"
    );

    let algo = Algo::new(AlgoType::Default);
    assert_eq!(algo.hash("hello".to_string()), "16156531084128653017");
  }
}
