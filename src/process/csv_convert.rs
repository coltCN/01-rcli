use std::fs;

use anyhow::Result;
use csv::Reader;
// use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::OutputFormat;

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "PascalCase")]
// struct Player {
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut result = Vec::with_capacity(128);
    for record in reader.records() {
        let record = record?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        result.push(json_value)
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&result)?,
        OutputFormat::Yaml => serde_yaml::to_string(&result)?,
    };
    fs::write(output, content)?;
    Ok(())
}
