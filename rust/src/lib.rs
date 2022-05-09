mod utils;
use utils::process::process;
use utils::workflow::Workflow;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_workflow(contents: &str) -> String {
  let workflow: Workflow = serde_yaml::from_str(&contents).unwrap();
  match process(&workflow) {
    Ok(plan) => match serde_json::to_string_pretty(&plan) {
      Ok(json) => json,
      Err(err) => format!("{:?}", err),
    },
    Err(e) => format!("{:?}", e),
  }
}