package plan

type Plan struct {
	Name     string    `json:"name"`
	Triggers []Trigger `json:"triggers"`
	Jobs     []Job     `json:"jobs"`
}

type Trigger struct {
	Kind WorkflowEvent `json:"kind"`
}

type WorkflowEvent string

const (
	WorkflowDispatch WorkflowEvent = "workflow_dispatch"
)

type Job struct {
	Name      string   `json:"name"`
	Identifer string   `json:"id"`
	Labels    []string `json:"labels"`
	Steps     []Step   `json:"steps"`
}

type Step struct {
	Type   StepType `json:"type"`
	Script *string  `json:"script,omitempty"`
}

type StepType string

const (
	RunStep StepType = "run"
)
