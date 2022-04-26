package process

func flattenMatrix(m map[string][]string) []map[string]string {
	var legs []map[string]string
	for k, v := range m {
		newSize := len(legs) * len(v)
		if len(legs) == 0 {
			newSize = len(v)
		}
		if newSize > 256 {
			panic("matrix too large")
		}
		newLegs := make([]map[string]string, 0, newSize)

		if len(legs) == 0 {
			// Add legs directly into map
			for _, j := range v {
				leg := map[string]string{k: j}
				newLegs = append(newLegs, leg)
			}
		} else {
			// For each new leg, copy all existing legs and add new value
			for _, oldLeg := range legs {
				for _, j := range v {
					leg := copyMap(oldLeg)
					leg[k] = j
					newLegs = append(newLegs, leg)
				}
			}
		}

		legs = newLegs
	}

	return legs
}

func copyMap(in map[string]string) map[string]string {
	out := make(map[string]string, len(in))
	for x, y := range in {
		out[x] = y
	}
	return out
}
