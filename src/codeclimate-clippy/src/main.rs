#![feature(custom_attribute)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate derive_new;

use std::process::Command;
use std::fs::File;
use std::io::Read;

use serde_json::{Deserializer, Value, map};

// The path to where we copy the source in the docker container
const DOCKER_SRC_PATH: &'static str = "/code-copy/";

#[derive(Serialize, new, PartialEq, Debug)]
struct Position {
    line: i64,
    column: i64,
}

#[derive(Serialize, new, PartialEq, Debug)]
struct Positions {
    begin: Position,
    end: Position,
}

#[derive(Serialize, new, PartialEq, Debug)]
struct Location {
    path: String,
    positions: Positions,
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
enum Severity {
    Info,
    Minor,
    Major,
    Critical,
    Blocker,
}

impl Severity {
    pub fn from_str(sev: &str) -> Option<Severity> {
        match sev {
            "note" => Some(Severity::Info),
            "help" => Some(Severity::Minor),
            "warning" => Some(Severity::Major),
            "error" => Some(Severity::Critical),
            "error: internal compiler error" => Some(Severity::Blocker),
            _ => None,
        }
    }
}

#[derive(new, Serialize, PartialEq, Debug)]
struct Issue<'issue> {
    #[new(value = r#""issue""#)]
    #[serde(rename="type")]
    itype: &'static str,
    check_name: &'issue str,
    description: String,
    categories: Vec<&'issue str>,
    location: Location,
    severity: Severity,
}

fn parse_check_name<'a>(data: &'a map::Map<String, Value>) -> &'a str {
    if let Some(children) = data["message"]["children"].as_array() {
        children[0]["message"]
            .as_str()
            .expect("Could not extract check_name")
    } else {
        panic!("Error parsing the check name!")
    }
}

fn parse_description(data: &map::Map<String, Value>) -> String {
    let mut message = data["message"]["message"]
        .as_str()
        .expect("Could not extract message")
        .to_owned();

    if let Some(children) = data["message"]["children"].as_array() {
        if children.len() >= 2 {
            message.push_str(", ");
            message.push_str(children[1]["message"]
                                 .as_str()
                                 .expect("Could not extract check_name"));
        }
    }

    message
}

fn parse_severity(data: &map::Map<String, Value>) -> Severity {
    Severity::from_str(data["message"]["level"]
                           .as_str()
                           .expect("Could not extract level"))
            .expect("Could not get severity from level")
}

fn get_categories(sev: &Severity) -> Vec<&'static str> {
    match sev {
        &Severity::Blocker |
        &Severity::Critical => vec!["Bug Risk"],
        _ => vec!["Clarity", "Performance"],
    }
}

fn parse_location<'a>(data: &'a map::Map<String, Value>) -> Location {
    let src_path = data["target"]["src_path"]
        .as_str()
        .expect("Could not extract src_path")
        .to_string()
        // make the path relative
        .replace(DOCKER_SRC_PATH, "");

    let spans = data["message"]["spans"]
        .as_array()
        .expect("Could not extract the spans array");

    let first = spans.first().expect("Spans should not be empty!");

    let line_start = first["line_start"]
        .as_i64()
        .expect("Could not exctract line_start");
    let line_end = first["line_end"]
        .as_i64()
        .expect("Could not exctract line_end");
    let column_start = first["column_start"]
        .as_i64()
        .expect("Could not exctract column_start");
    let column_end = first["column_end"]
        .as_i64()
        .expect("Could not exctract column_end");

    let positions = Positions::new(Position::new(line_start, column_start),
                                   Position::new(line_end, column_end));

    Location::new(src_path, positions)
}

fn parse_issue<'a>(data: &'a map::Map<String, Value>) -> Issue<'a> {
    let check_name = parse_check_name(data);
    let description = parse_description(data);
    let severity = parse_severity(data);
    let categories = get_categories(&severity);
    let location = parse_location(data);

    Issue::new(check_name, description, categories, location, severity)
}

fn print_issue<'a>(issue: &'a Issue) {
    println!("{}\0",
             serde_json::to_string_pretty(&issue).expect("Error creating json from issue struct"))
}

fn get_include_paths() -> Vec<String> {
    if let Ok(mut config) = File::open("/config.js") {

        let mut content = String::new();
        config
            .read_to_string(&mut content)
            .expect("Error reading /config.js");

        let json: Value = serde_json::from_str(&content).expect("Error converting content to json");
        json["include_paths"]
            .as_array()
            .expect("Could not get include_paths")
            .iter()
            .map(|s| {
                     let mut path = DOCKER_SRC_PATH.to_string();
                     path.push_str(s.as_str().expect("Error extracting include path"));
                     path
                 })
            .collect::<Vec<String>>()
    } else {
        vec![DOCKER_SRC_PATH.to_string()]
    }
}

fn is_in_include_path(include_paths: &Vec<String>, data: &map::Map<String, Value>) -> bool {
    let src_path = data["target"]["src_path"]
        .as_str()
        .expect("Could not extract src_path");

    include_paths.iter().any(|path| src_path.contains(path))
}

fn main() {
    let include_paths = get_include_paths();
    let output = Command::new("cargo")
        .args(&["clippy", "--message-format", "json", "-q"])
        .output()
        .expect("Error executing clippy")
        .stdout;

    let stream = Deserializer::from_slice(output.as_slice()).into_iter::<Value>();

    for value in stream {
        if let Ok(json) = value {
            if let Some(object) = json.as_object() {
                if object.contains_key("message") {
                    if is_in_include_path(&include_paths, object) {
                        let issue = parse_issue(object);
                        print_issue(&issue);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;

    const TEST_ISSUE: &'static str = r##"{"message":{"children":[{"children":[],"code":null,
"level":"note","message":"#[deny(unused_io_amount)] on by default","rendered":null,
"spans":[]},{"children":[],"code":null,"level":"help",
"message":
"for further information visit https://github.com/Manishearth/rust-clippy/wiki#unused_io_amount",
    "rendered":null,"spans":[]}],"code":null,"level":"error",
"message":"handle written amount returned or use `Write::write_all` instead",
"rendered":null,"spans":
[{"byte_end":8079,"byte_start":7989,"column_end":18,"column_start":5,"expansion":null,
  "file_name":"tests/integration.rs","is_primary":true,"label":null,"line_end":239,
  "line_start":237,"suggested_replacement":null,"text":
  [{"highlight_end":13,"highlight_start":5,"text":
    "    url_file"},
   {"highlight_end":64,"highlight_start":1,
    "text":"        .write(portal_address_to_redirect_url(addr).as_bytes())"},
   {"highlight_end":18,"highlight_start":1,"text":"        .unwrap();"}]}]},
"package_id":"sentry 0.1.0 (path+file:///home/bastian/projects/superscale/sentry-rs)",
"reason":"compiler-message","target":{"crate_types":["bin"],"kind":["test"],
    "name":"integration",
    "src_path":"/code-copy/tests/integration.rs"}}"##;

    #[test]
    fn test_issue_parse() {
        let json: serde_json::Value =
            serde_json::from_str(TEST_ISSUE).expect("Error parsing the test issue");


        let object = json.as_object().expect("Could not get json object");

        let issue = parse_issue(object);

        let location = Location::new("tests/integration.rs".to_string(),
                                     Positions::new(Position::new(237, 5), Position::new(239, 18)));

        let expected = Issue::new("#[deny(unused_io_amount)] on by default",
                                  "handle written amount returned or use `Write::write_all` \
            instead, for further information visit \
            https://github.com/Manishearth/rust-clippy/wiki#unused_io_amount"
                                          .to_string(),
                                  get_categories(&Severity::Critical),
                                  location,
                                  Severity::Critical);

        assert_eq!(issue, expected);
    }
}
