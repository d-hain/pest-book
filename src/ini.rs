use std::{collections::HashMap, fs};

use pest::Parser;
use pest_derive::Parser;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "ini.pest"]
pub(crate) struct IniParser;

#[allow(dead_code)]
pub(crate) fn ini_example() {
    let unparsed_file = fs::read_to_string("config.ini").expect("Could not read file");

    let file = IniParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    let mut current_section_name = "";

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                let mut inner_rules = line.into_inner(); // { name }
                current_section_name = inner_rules.next().unwrap().as_str();
            }
            Rule::property => {
                let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

                let name = inner_rules.next().unwrap().as_str();
                let value = inner_rules.next().unwrap().as_str();

                let section = properties.entry(current_section_name).or_default();
                section.insert(name, value);
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    println!("{:#?}", properties);
}
