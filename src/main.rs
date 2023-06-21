extern crate serde_json;
extern crate serde_yaml;

use std::io::{self, Read};
use std::error::Error;
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&buffer)?;

    // Convert the JSON object to a YAML String.
    let yaml = serde_yaml::to_string(&v)?;

    // Print the YAML string to stdout.
    println!("{}", yaml);

    Ok(())
}
