package schema

import "gopkg.in/yaml.v3"

type WorkflowRoot struct {
	Raw  *yaml.Node     `json:"-"`
	Name string         `yaml:"name"`
	On   string         `yaml:"on"`
	Jobs map[string]Job `yaml:"jobs"`
}

type Job struct {
	Labels   string   `yaml:"runs-on"`
	Steps    []Step   `yaml:"steps"`
	Strategy Strategy `yaml:"strategy"`
}

type Strategy struct {
	Matrix map[string][]string `yaml:"matrix"`
}

type Step struct {
	Script string `yaml:"run"`
}
