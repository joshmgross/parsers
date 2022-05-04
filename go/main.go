package main

import (
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	"gopkg.in/yaml.v3"

	"github.com/joshmgross/parsers/go/process"
	"github.com/joshmgross/parsers/go/schema"
)

func main() {
	if err := realMain(); err != nil {
		fmt.Fprintf(os.Stderr, "%s\n", err)
		os.Exit(1)
	}
}

func realMain() error {
	var inputDir string
	flag.StringVar(&inputDir, "in", "", "directory of workflows to parse")
	var outputDir string
	flag.StringVar(&outputDir, "out", "", "directory to write generated plans")
	flag.Parse()

	if inputDir == "" {
		return errors.New("Expected workflow directory input `-in`")
	}
	if outputDir == "" {
		return errors.New("Expected output directory `-out`")
	}

	dir, err := os.ReadDir(inputDir)
	if err != nil {
		return err
	}

	for _, file := range dir {
		if file.IsDir() {
			continue
		}

		if filepath.Ext(file.Name()) != ".yaml" {
			return fmt.Errorf("Unexpected non-YAML file %q", file.Name())
		}

		name := strings.TrimSuffix(file.Name(), ".yaml")
		planFile := name + ".json"
		plan, err := generatePlan(filepath.Join(inputDir, file.Name()))
		if err != nil {
			return err
		}

		err = os.WriteFile(filepath.Join(outputDir, planFile), plan, 0644)
		if err != nil {
			return err
		}
	}

	return nil
}

func generatePlan(path string) ([]byte, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}

	wf, err := parse(file)
	if err != nil {
		return nil, err
	}

	p := process.Process(wf)

	return json.MarshalIndent(p, "", "  ")
}

func parse(in io.Reader) (schema.WorkflowRoot, error) {
	var root schema.WorkflowRoot
	decoder := yaml.NewDecoder(in)
	err := decoder.Decode(&root)
	return root, err
}
