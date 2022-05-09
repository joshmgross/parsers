use super::matrix::flatten_matrix;
use super::plan::*;
use super::workflow::Workflow;

use std::str::FromStr;

pub fn process(workflow: &Workflow) -> Result<Plan, &str> {
    let event = WorkflowEvent::from_str(&workflow.on);

    let event = match event {
        Ok(event) => event,
        Err(_) => return Err("Could not parse event"),
    };

    let mut plan = Plan {
        name: workflow.name.to_owned(),
        triggers: vec![Trigger { kind: event }],
        jobs: vec![],
    };

    for (name, job) in &workflow.jobs {
        match &job.strategy {
            Some(strategy) => {
                let legs = flatten_matrix(&strategy.matrix).unwrap();
                for leg in &legs {
                    // TODO: Support cloning a step so we don't need to recreate the steps for each job
                    let mut steps = vec![];
                    for step in &job.steps {
                        let plan_step = super::plan::Step {
                            step_type: StepType::RunStep,
                            script: step.run.clone(),
                        };
                        steps.push(plan_step);
                    }

                    // TODO: The name and identifier should change within the context of a matrix
                    let plan_job = Job {
                        name: name.to_owned(),
                        id: name.to_owned(),
                        labels: job.runs_on.labels.clone(),
                        steps: steps,
                        matrix: Some(leg.clone()),
                    };

                    plan.jobs.push(plan_job);
                }
            }
            None => {
                let mut steps = vec![];
                for step in &job.steps {
                    let plan_step = super::plan::Step {
                        step_type: StepType::RunStep,
                        script: step.run.clone(),
                    };
                    steps.push(plan_step);
                }

                let plan_job = Job {
                    name: name.to_owned(),
                    id: name.to_owned(),
                    labels: job.runs_on.labels.clone(),
                    steps: steps,
                    matrix: None,
                };

                plan.jobs.push(plan_job);
            }
        }
    }

    Ok(plan)
}
