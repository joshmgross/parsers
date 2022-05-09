use std::collections::BTreeMap as Map;

pub fn flatten_matrix(map: &Map<String, Vec<String>>) -> Result<Vec<Map<String, String>>, String> {
    let mut result: Vec<Map<String, String>> = Vec::new();
    for (k, v) in map {
        let mut new_size = result.len() * v.len();
        if result.len() == 0 {
            new_size = v.len();
        }
        if new_size > 256 {
            return Err(String::from("Too many combinations"));
        }
        let mut new_legs: Vec<Map<String, String>> = Vec::with_capacity(new_size);
        if result.len() == 0 {
            // Add legs directly into map
            for leg in v {
                let leg = Map::from([(k.to_string(), leg.to_string())]);
                new_legs.push(leg);
            }
        } else {
            // For each new leg, copy all existing legs and add new value
            for old_leg in &result {
                for leg in v {
                    let mut new_leg = old_leg.clone();
                    new_leg.insert(k.clone(), leg.clone());
                    new_legs.push(new_leg);
                }
            }
        }

        result = new_legs;
    }
    Ok(result)
}

// TODO: Figure out where this test should go
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_matrix() {
        let mut map: Map<String, Vec<String>> = Map::new();
        map.insert(
            "node".to_string(),
            vec!["10".to_string(), "11".to_string(), "12".to_string()],
        );
        map.insert(
            "os".to_string(),
            vec!["ubuntu-latest".to_string(), "windows-latest".to_string()],
        );
        let combinations = flatten_matrix(&map).unwrap();
        assert_eq!(combinations.len(), 6);

        let expected = vec![
            Map::from([
                ("node".to_string(), "10".to_string()),
                ("os".to_string(), "ubuntu-latest".to_string()),
            ]),
            Map::from([
                ("node".to_string(), "10".to_string()),
                ("os".to_string(), "windows-latest".to_string()),
            ]),
            Map::from([
                ("node".to_string(), "11".to_string()),
                ("os".to_string(), "ubuntu-latest".to_string()),
            ]),
            Map::from([
                ("node".to_string(), "11".to_string()),
                ("os".to_string(), "windows-latest".to_string()),
            ]),
            Map::from([
                ("node".to_string(), "12".to_string()),
                ("os".to_string(), "ubuntu-latest".to_string()),
            ]),
            Map::from([
                ("node".to_string(), "12".to_string()),
                ("os".to_string(), "windows-latest".to_string()),
            ]),
        ];
        assert_eq!(combinations, expected);
    }
}
