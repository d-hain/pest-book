use std::fs;

use pest::{error::Error, Parser};
use pest_derive::Parser;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "json.pest"]
pub(crate) struct JsonParser;

enum JsonValue<'a> {
    Object(Vec<(&'a str, JsonValue<'a>)>),
    Array(Vec<JsonValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

#[allow(dead_code)]
pub(crate) fn json_example() {
    let unparsed_file = fs::read_to_string("data.json").expect("Could not read file");

    let json: JsonValue = parse_json_file(&unparsed_file).expect("unsuccessful parse");

    println!("{}", serialize_jsonvalue(&json));
}

fn parse_json_file(file: &str) -> Result<JsonValue, Error<Rule>> {
    let json = JsonParser::parse(Rule::json, file)?.next().unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> JsonValue {
        match pair.as_rule() {
            Rule::object => JsonValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => JsonValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => JsonValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => JsonValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JsonValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JsonValue::Null,
            Rule::json
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::WHITESPACE => unreachable!(),
        }
    }

    Ok(parse_value(json))
}

/// A cheap mans `Display` implementation.
fn serialize_jsonvalue(val: &JsonValue) -> String {
    use JsonValue::*;

    match val {
        Object(obj) => {
            let contents: Vec<_> = obj
                .iter()
                .map(|(name, value)| format!("\"{}\":{}", name, serialize_jsonvalue(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Array(arr) => {
            let contents: Vec<_> = arr.iter().map(serialize_jsonvalue).collect();
            format!("[{}]", contents.join(","))
        }
        String(str) => format!("\"{}\"", str),
        Number(num) => format!("{}", num),
        Boolean(bool) => format!("{}", bool),
        Null => "null".to_owned(),
    }
}
