package process

import (
	"strings"

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
		pj := plan.Job{
			Name:      name,
			Identifer: name,
			Labels:    strings.Split(job.Labels, ","),
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

	return p
}
