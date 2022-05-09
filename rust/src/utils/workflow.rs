use std::collections::BTreeMap as Map;
use std::fmt;

use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub on: String,
    pub jobs: Map<String, Job>,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    #[serde(rename = "runs-on")]
    pub runs_on: RunsOn,
    pub steps: Vec<Step>,
    pub strategy: Option<Strategy>,
}

#[derive(Debug)]
pub struct RunsOn {
    pub labels: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Strategy {
    pub matrix: Map<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Step {
    pub name: Option<String>,
    pub uses: Option<String>,
    pub with: Option<Map<String, String>>,
    pub run: Option<String>,
}

impl<'de> Deserialize<'de> for RunsOn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RunsOnVisitor;

        impl<'de> Visitor<'de> for RunsOnVisitor {
            type Value = RunsOn;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or sequence of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<RunsOn, E> {
                Ok(RunsOn {
                    labels: vec![value.to_owned()],
                })
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<RunsOn, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut labels = vec![];
                while let Some(label) = seq.next_element()? {
                    labels.push(label);
                }
                Ok(RunsOn { labels })
            }
        }

        deserializer.deserialize_any(RunsOnVisitor)
    }
}
