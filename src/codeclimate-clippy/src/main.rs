#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate derive_new;

use std::process::Command;

use serde_json::{Deserializer, Value, map};

#[derive(Serialize,new)]
struct Position {
    line: usize,
    column: usize,
}

#[derive(Serialize,new)]
struct Positions {
    begin: Position,
    end: Position,
}

#[derive(Serialize,new)]
struct Location<'location> {
    path: &'location str,
    positions: Positions,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum Severity {
    Info,
    Minor,
    Critical,
}

#[derive(Serialize,new)]
struct Issue<'issue> {
    itype: &'issue str,
    check_name: &'issue str,
    description: &'issue str,
    categories: Vec<&'issue str>,
    location: Location<'issue>,
    severity: Severity,
}

fn print_issue(data: &map::Map<String, Value>) {}

fn main() {
    let mut output = Command::new("cargo")
        .args(&["clippy", "--message-format", "json", "-q"])
        .output()
        .expect("Error executing clippy")
        .stdout;

    let stream = Deserializer::from_slice(output.as_slice()).into_iter::<Value>();

    for value in stream {
        if let Ok(json) = value {
            if let Some(msg) = json["message"].as_object() {
                println!("{:?}", msg);
            }
        }
    }
}
