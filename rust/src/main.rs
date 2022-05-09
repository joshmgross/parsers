use serde_json;
use std::fs;
use std::fs::File;

mod utils;
use utils::process::process;
use utils::workflow::Workflow;

fn main() {
    let workflows = vec!["manual", "label-array", "matrix"];
    for workflow_name in workflows {
        let contents = fs::read_to_string(format!("../workflows/{}.yaml", workflow_name))
            .expect("Something went wrong reading the file");
        let workflow: Workflow = serde_yaml::from_str(&contents).unwrap();

        let plan = process(&workflow).unwrap();

        let file = File::create(format!("../generated/rust/{}.json", workflow_name)).unwrap();
        serde_json::to_writer_pretty(file, &plan).unwrap();
    }
}
