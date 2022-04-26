package process

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_FlattenMatrix(t *testing.T) {
	tests := []struct {
		name string
		in   map[string][]string
		out  []map[string]string
	}{
		{
			name: "simple",
			in:   map[string][]string{"node": {"10", "11", "12"}},
			out: []map[string]string{
				{"node": "10"},
				{"node": "11"},
				{"node": "12"},
			},
		},
		{
			name: "2x3",
			in: map[string][]string{
				"node": {"10", "11", "12"},
				"os":   {"ubuntu-latest", "windows-latest"},
			},
			out: []map[string]string{
				{"node": "10", "os": "ubuntu-latest"},
				{"node": "10", "os": "windows-latest"},
				{"node": "11", "os": "ubuntu-latest"},
				{"node": "11", "os": "windows-latest"},
				{"node": "12", "os": "ubuntu-latest"},
				{"node": "12", "os": "windows-latest"},
			},
		},
		{
			name: "3x2",
			in: map[string][]string{
				"a": {"0", "1"},
				"b": {"2", "3"},
				"c": {"4", "5"},
			},
			out: []map[string]string{
				{"a": "0", "b": "2", "c": "4"},
				{"a": "0", "b": "2", "c": "5"},
				{"a": "0", "b": "3", "c": "4"},
				{"a": "0", "b": "3", "c": "5"},
				{"a": "1", "b": "2", "c": "4"},
				{"a": "1", "b": "2", "c": "5"},
				{"a": "1", "b": "3", "c": "4"},
				{"a": "1", "b": "3", "c": "5"},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// This is currently order dependent
			require.Equal(t, tt.out, flattenMatrix(tt.in))
		})
	}
}
