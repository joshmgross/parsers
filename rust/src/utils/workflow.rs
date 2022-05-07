use std::collections::BTreeMap as Map;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use void::Void;

#[derive(Debug, Deserialize)]
pub struct Workflow {
  pub name: String,
  pub on: String,
  pub jobs: Map<String, Job>,
}

#[derive(Debug, Deserialize)]
pub struct Job {
  #[serde(rename = "runs-on")]
  #[serde(deserialize_with = "string_or_struct")]
  pub runs_on: RunsOn,
  pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
pub struct RunsOn {
  pub labels: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Step {
  pub name: Option<String>,
  pub uses: Option<String>,
  pub with: Option<Map<String, String>>,
  pub run: Option<String>,
}

impl FromStr for RunsOn {
  type Err = Void;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(RunsOn {
      labels: vec![s.to_owned()],
    })
  }
}

// From https://serde.rs/string-or-struct.html
fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
  T: Deserialize<'de> + FromStr<Err = Void>,
  D: Deserializer<'de>,
{
  // This is a Visitor that forwards string types to T's `FromStr` impl and
  // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
  // keep the compiler from complaining about T being an unused generic type
  // parameter. We need T in order to know the Value type for the Visitor
  // impl.
  struct StringOrStruct<T>(PhantomData<fn() -> T>);

  impl<'de, T> Visitor<'de> for StringOrStruct<T>
  where
    T: Deserialize<'de> + FromStr<Err = Void>,
  {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("string or map")
    }

    fn visit_str<E>(self, value: &str) -> Result<T, E>
    where
      E: de::Error,
    {
      Ok(FromStr::from_str(value).unwrap())
    }

    fn visit_map<M>(self, map: M) -> Result<T, M::Error>
    where
      M: MapAccess<'de>,
    {
      // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
      // into a `Deserializer`, allowing it to be used as the input to T's
      // `Deserialize` implementation. T then deserializes itself using
      // the entries from the map visitor.
      Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
    }
  }

  deserializer.deserialize_any(StringOrStruct(PhantomData))
}
