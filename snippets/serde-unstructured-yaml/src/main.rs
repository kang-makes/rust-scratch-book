use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde;
use serde_yaml::{self, Value};

const TEST_MANIFEST: &str = r#"
api_version: some.kubernetes.crd/v1alpha1
kind: Object
metadata:
  name: test
spec:
  string: env_var1-env_var1-env_var2-env_var2-env_var1-env_var2
  bool: true
  values:
    valuesInside1: ${env_var1}
    valuesInside2: ${env_var2}
    ${env_var1}: Do not scape me
    ${env_var2}: Do not scape me
  really:
    deep:
      sequence:
      - env_var1
      - env_var2
      - env_var1
      - env_var1
      - env_var2
      - env_var2
      - env_var1-env_var1-env_var2-env_var2-env_var1-env_var2
      - object:
          test:
            key: env_var1-env_var1-env_var2-env_var2-env_var1-env_var2
            env_var1-env_var1-env_var2-env_var2-env_var1-env_var2: value
"#;

#[derive(Debug, Serialize, Deserialize)]
struct Object {
  api_version: String,
  kind: String,
  metadata: ObjectMetadata,
  spec: Value,
}
#[derive(Debug, Serialize, Deserialize)]
struct ObjectMetadata {
    name: String,
    #[serde(default)]
    namespace: String,
}


fn main() {
  let replacements: HashMap<&str, &str> = HashMap::from(
    [
      ("env_var1", "REPLACED-1"),
      ("env_var2", "REPLACED-2")
    ]
  );

  let udata: Value = serde_yaml::from_str(TEST_MANIFEST).unwrap(); // Part of the POC as if it came from a demarshaller upstream
  let gvk_muted = mute_values(udata, replacements); // Mute the YAML without knowing its structure
  let object: Object = serde_yaml::from_value::<Object>(gvk_muted).unwrap(); // Return the mutated yaml to a rust dat
  println!("{:#?}", object);
}

fn mute_values(values: Value, replacements: HashMap<&str, &str>) -> Value {
    match values {
        Value::Mapping(m) => Value::Mapping(m.into_iter().map(|(k, v)| { (k, mute_values(v, replacements.clone())) }).collect()),
        Value::Sequence(seq) => seq.iter().map(|v| {mute_values(v.clone(), replacements.clone())}).collect(),
        Value::String(st) => Value::String(template_replacements(st, replacements.clone())),
        _ => values,
    }
}

fn template_replacements(mut s: String, replacements: HashMap<&str, &str>) -> String {
  for (k, v) in replacements {
    s = s.replace(k, v);
  }

  s
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_yaml::{self, Value};

    #[test]
    fn test_object() {
      const HAVE: &str = r#"
      api_version: some.kubernetes.crd/v1alpha1
      kind: Object
      metadata:
        name: test
      spec:
        values:
          valuesInside1: ${env_var1}
          valuesInside2: ${env_var2}
          ${env_var1}: Do not scape me
          ${env_var2}: Do not scape me
        string: env_var1-env_var1-env_var2-env_var2-env_var1-env_var2
      "#;
      const WANT: &str = r#"
      api_version: some.kubernetes.crd/v1alpha1
      kind: Object
      metadata:
        name: test
      spec:
        values:
          valuesInside1: ${OBJECTS-1}
          valuesInside2: ${OBJECTS-2}
          ${env_var1}: Do not scape me
          ${env_var2}: Do not scape me
        string: OBJECTS-1-OBJECTS-1-OBJECTS-2-OBJECTS-2-OBJECTS-1-OBJECTS-2
      "#;

      let have: Value = serde_yaml::from_str(HAVE).unwrap();
      let want: Value = serde_yaml::from_str(WANT).unwrap();
      let replacements: HashMap<&str, &str> = HashMap::from(
        [
          ("env_var1", "OBJECTS-1"),
          ("env_var2", "OBJECTS-2")
        ]
      );

      let muted: Value = super::mute_values(have, replacements);
      assert_eq!(muted, want);
    }

    #[test]
    fn test_mapping() {
      const HAVE: &str = r#"
      api_version: some.kubernetes.crd/v1alpha1
      kind: Object
      metadata:
        name: test
      spec:
        - ${env_var1}
        - ${env_var2}
        - env_var1-env_var1-env_var2-env_var2-env_var1-env_var2
      "#;
      const WANT: &str = r#"
      api_version: some.kubernetes.crd/v1alpha1
      kind: Object
      metadata:
        name: test
      spec:
      - ${MAPPING-1}
      - ${MAPPING-2}
      - MAPPING-1-MAPPING-1-MAPPING-2-MAPPING-2-MAPPING-1-MAPPING-2
      "#;

      let have: Value = serde_yaml::from_str(HAVE).unwrap();
      let want: Value = serde_yaml::from_str(WANT).unwrap();
      let replacements: HashMap<&str, &str> = HashMap::from(
        [
          ("env_var1", "MAPPING-1"),
          ("env_var2", "MAPPING-2")
        ]
      );

      let muted: Value = super::mute_values(have, replacements);
      assert_eq!(muted, want);
    }

    #[test]
    fn test_mixed() {
      const HAVE: &str = r#"
      api_version: some.kubernetes.crd/v1alpha1
      kind: Object
      metadata:
        name: test
      spec:
        - valuesInside1: ${env_var1}
        - valuesInside2: ${env_var2}
        - ${env_var1}: Do not scape me
        - ${env_var2}: Do not scape me
        - string: env_var1-env_var1-env_var2-env_var2-env_var1-env_var2
      "#;
      const WANT: &str = r#"
      api_version: some.kubernetes.crd/v1alpha1
      kind: Object
      metadata:
        name: test
      spec:
      - valuesInside1: ${MIXED-1}
      - valuesInside2: ${MIXED-2}
      - ${env_var1}: Do not scape me
      - ${env_var2}: Do not scape me
      - string: MIXED-1-MIXED-1-MIXED-2-MIXED-2-MIXED-1-MIXED-2
      "#;

      let have: Value = serde_yaml::from_str(HAVE).unwrap();
      let want: Value = serde_yaml::from_str(WANT).unwrap();
      let replacements: HashMap<&str, &str> = HashMap::from(
        [
          ("env_var1", "MIXED-1"),
          ("env_var2", "MIXED-2")
        ]
      );

      let muted: Value = super::mute_values(have, replacements);
      assert_eq!(muted, want);
    }

}