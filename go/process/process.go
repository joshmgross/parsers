package process

import (
	"github.com/joshmgross/parsers/go/plan"
	"github.com/joshmgross/parsers/go/schema"
)

func Process(wf schema.WorkflowRoot) plan.Plan {
	p := plan.Plan{
		Name: wf.Name,
	}

	p.Triggers = []plan.Trigger{
		{Kind: plan.WorkflowEvent(wf.On)},
	}

	p.Jobs = make([]plan.Job, 0, len(wf.Jobs))
	for name, job := range wf.Jobs {
		if len(job.Strategy.Matrix) == 0 {
			pj := plan.Job{
				Name:      name,
				Identifer: name,
				Labels:    job.RunsOn.Labels,
			}

			pj.Steps = make([]plan.Step, 0, len(job.Steps))
			for _, step := range job.Steps {
				pj.Steps = append(pj.Steps, plan.Step{
					Type:   plan.RunStep,
					Script: &step.Script,
				})
			}
			p.Jobs = append(p.Jobs, pj)
		} else {
			for _, matrixLeg := range flattenMatrix(job.Strategy.Matrix) {
				// TODO: The name and identifier should change within the context of a matrix
				pj := plan.Job{
					Name:      name,
					Identifer: name,
					Labels:    job.RunsOn.Labels,
					Matrix:    matrixLeg,
				}

				pj.Steps = make([]plan.Step, 0, len(job.Steps))
				for _, step := range job.Steps {
					pj.Steps = append(pj.Steps, plan.Step{
						Type:   plan.RunStep,
						Script: &step.Script,
					})
				}
				p.Jobs = append(p.Jobs, pj)
			}
		}
	}

	return p
}
