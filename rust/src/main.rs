use std::fs;
use std::fs::File;

mod utils;

use utils::workflow::Workflow;
use utils::process::process;
use serde_json;

fn main() {
  let contents =
    fs::read_to_string("../workflows/manual.yaml").expect("Something went wrong reading the file");
  let workflow: Workflow = serde_yaml::from_str(&contents).unwrap();

  let plan = process(&workflow).unwrap();

  let file = File::create("../generated/rust/manual.json").unwrap();
  serde_json::to_writer_pretty(file, &plan).unwrap();
}
