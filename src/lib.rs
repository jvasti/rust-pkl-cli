use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use new_pkl::{Pkl, PklResult, PklValue};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::fs::File;
use std::io::{stdout, Write};

pub fn parse(input: &str) -> PklResult<Pkl> {
    let mut pkl = Pkl::new();
    pkl.parse(input)?;
    Ok(pkl)
}
#[derive(Debug)]
pub struct SortedPklValue<'a>(pub &'a PklValue<'a>);

impl<'a> Serialize for SortedPklValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            PklValue::Null => serializer.serialize_unit(),
            PklValue::Bool(b) => serializer.serialize_bool(*b),
            PklValue::Float(f) => serializer.serialize_f64(*f),
            PklValue::Int(i) => serializer.serialize_i64(*i),
            PklValue::String(s) => serializer.serialize_str(s),
            PklValue::List(vec) => {
                let mut seq = serializer.serialize_seq(Some(vec.len()))?;
                for item in vec {
                    seq.serialize_element(&SortedPklValue(item))?;
                }
                seq.end()
            }
            PklValue::Object(map) => {
                // Collect and sort the map entries
                let mut entries: Vec<_> = map.iter().collect();
                entries.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

                let mut map_ser = serializer.serialize_map(Some(entries.len()))?;
                for (k, v) in entries {
                    map_ser.serialize_entry(k, &SortedPklValue(v))?;
                }
                map_ser.end()
            }
            PklValue::ClassInstance(class_name, properties) => {
                // Collect and sort the properties
                let mut entries: Vec<_> = properties.iter().collect();
                entries.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

                let mut map_ser = serializer.serialize_map(Some(entries.len() + 1))?;
                // Serialize the class name first
                map_ser.serialize_entry("__class__", class_name)?;
                for (k, v) in entries {
                    map_ser.serialize_entry(k, &SortedPklValue(v))?;
                }
                map_ser.end()
            }
            PklValue::Duration(duration) => duration.serialize(serializer),
            PklValue::DataSize(data_size) => data_size.serialize(serializer),
        }
    }
}

pub fn read_input_file(path: &Utf8PathBuf) -> Result<String> {
    if *path == *"-" {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;
        Ok(buffer)
    } else {
        std::fs::read_to_string(path).context(format!("Failed to read file '{}'", path))
    }
}

pub fn write_output(path: &Option<Utf8PathBuf>, content: &str) -> Result<()> {
    match path {
        Some(path) => {
            let mut output_file = File::create_new(path)
                .context(format!("Failed to create new output file '{}'", path))?;
            output_file.write_all(content.as_bytes()).context(format!(
                "Failed to write to output file:\nContent: {}",
                content
            ))?;
        }
        None => {
            stdout()
                .write_all(content.as_bytes())
                .context(format!("Failed to write to stdout:\nContent: {}", content))?;
            stdout().write_all(b"\n")?;
        }
    }

    Ok(())
}
