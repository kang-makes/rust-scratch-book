use std::error::Error;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::Chart;
use serde_yaml::Value;

pub fn chart_from_file(override_path: &str, pb: PathBuf) -> Result<Chart, Box<dyn Error>> {
    let chart_file = File::open(pb.clone())?;
    let value: serde_yaml::Value = serde_yaml::from_reader(chart_file)?;

    let override_: PathBuf = Path::new(override_path).join(pb);
    if !override_.exists() || !override_.is_file() {
        return Ok(serde_yaml::from_value::<Chart>(value)?);
    }

    let override_file = File::open(override_)?;
    let mut override_value: serde_yaml::Value = serde_yaml::from_reader(override_file)?;

    merge(&value, &mut override_value);

    Ok(serde_yaml::from_value::<Chart>(override_value)?)
}

fn merge(src: &Value, dst: &mut Value) {
    match (src, dst) {
        (Value::Mapping(src), Value::Mapping(dst)) => {
            for (key, sval) in src {
                if let Some(dval) = dst.get_mut(key) {
                    merge(sval, dval)
                } else {
                    dst.insert(key.clone(), sval.clone());
                }
            }
        }

        (Value::Sequence(src), Value::Sequence(dst)) => {
            dst.extend_from_slice(src);
        }

        (src, dst) => *dst = src.clone(),
    }
}
