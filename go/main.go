package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	"gopkg.in/yaml.v3"

	"github.com/joshmgross/parsers/go/process"
	"github.com/joshmgross/parsers/go/schema"
)

const workflowDirectory = "../workflows"

func main() {
	dir, err := os.ReadDir(workflowDirectory)
	if err != nil {
		panic(err)
	}

	for _, file := range dir {
		if file.IsDir() {
			continue
		}

		if filepath.Ext(file.Name()) != ".yaml" {
			continue
		}

		name := strings.TrimSuffix(file.Name(), ".yaml")
		planFile := name + ".json"
		compare(file.Name(), planFile)
	}
}

func compare(workflowFile, planFile string) {
	file, err := os.Open(filepath.Join(workflowDirectory, workflowFile))
	if err != nil {
		panic(err)
	}

	wf, err := parse(file)
	if err != nil {
		panic(err)
	}

	p := process.Process(wf)

	j, err := json.MarshalIndent(p, "", "  ")
	if err != nil {
		panic(err)
	}

	plan, err := os.ReadFile(filepath.Join(workflowDirectory, planFile))
	if err != nil {
		panic(err)
	}

	if string(plan) != string(j) {
		fmt.Printf("Generated plan mismatch for %s\n", workflowFile)
		fmt.Println("Expected:")
		fmt.Println(string(plan))
		fmt.Println("Actual:")
		fmt.Println(string(j))
	} else {
		fmt.Printf("Generated plan matches for %s\n", workflowFile)
	}
}

func parse(in io.Reader) (schema.WorkflowRoot, error) {
	var root schema.WorkflowRoot
	decoder := yaml.NewDecoder(in)
	err := decoder.Decode(&root)
	return root, err
}
