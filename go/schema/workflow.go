package schema

import (
	"fmt"

	"gopkg.in/yaml.v3"
)

type WorkflowRoot struct {
	Raw  *yaml.Node     `json:"-"`
	Name string         `yaml:"name"`
	On   string         `yaml:"on"`
	Jobs map[string]Job `yaml:"jobs"`
}

type Job struct {
	RunsOn   RunsOn   `yaml:"runs-on"`
	Steps    []Step   `yaml:"steps"`
	Strategy Strategy `yaml:"strategy"`
}

type Strategy struct {
	Matrix map[string][]string `yaml:"matrix"`
}

type Step struct {
	Script string `yaml:"run"`
}

type RunsOn struct {
	Labels []string
}

func (r *RunsOn) UnmarshalYAML(value *yaml.Node) error {
	var l []string
	switch value.Kind {
	case yaml.SequenceNode:
		if err := value.Decode(&l); err != nil {
			return err
		}
	case yaml.ScalarNode:
		var s string
		if err := value.Decode(&s); err != nil {
			return err
		}
		l = append(l, s)
	default:
		return fmt.Errorf("Unexpected value type: %v (%d:%d", value.Kind, value.Line, value.Column)
	}
	r.Labels = l
	return nil
}
