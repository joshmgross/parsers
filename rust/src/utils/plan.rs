use serde::Serialize;
use std::collections::BTreeMap as Map;
use std::str::FromStr;

#[derive(Debug, Serialize)]
pub struct Plan {
    pub name: String,
    pub triggers: Vec<Trigger>,
    pub jobs: Vec<Job>,
}

#[derive(Debug, Serialize)]
pub struct Trigger {
    pub kind: WorkflowEvent,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowEvent {
    WorkflowDispatch,
    Push,
}

#[derive(Debug, Serialize)]
pub struct Job {
    pub name: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Map<String, String>>,
    pub labels: Vec<String>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Serialize)]
pub struct Step {
    #[serde(rename = "type")]
    pub step_type: StepType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum StepType {
    #[serde(rename = "run")]
    RunStep,
}

impl FromStr for WorkflowEvent {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "workflow_dispatch" => Ok(WorkflowEvent::WorkflowDispatch),
            "push" => Ok(WorkflowEvent::Push),
            _ => Err(()),
        }
    }
}

impl FromStr for StepType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "run" => Ok(StepType::RunStep),
            _ => Err(()),
        }
    }
}
