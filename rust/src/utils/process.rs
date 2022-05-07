use super::workflow::Workflow;
use super::plan::*;

use std::str::FromStr;

pub fn process(workflow: &Workflow) -> Result<Plan, &str> {
  let event = WorkflowEvent::from_str(&workflow.on);

  let event = match event {
    Ok(event) => event,
    Err(_) => return Err("Could not parse event"),
  };

  let mut plan = Plan {
    name: workflow.name.to_owned(),
    triggers: vec![Trigger {kind: event}],
    jobs: vec![],
  };

  for (name, job) in &workflow.jobs {
    let mut steps = vec![];
    for step in &job.steps {
      let plan_step = super::plan::Step {
        step_type: StepType::RunStep,
        script: step.run.clone(),
      };
      steps.push(plan_step);
    }
    let job = Job {
      name: name.to_owned(),
      id: name.to_owned(),
      labels: job.runs_on.labels.clone(),
      steps: steps,
    };

    plan.jobs.push(job);
  }

  Ok(plan)
}