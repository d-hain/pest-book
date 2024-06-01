use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "csv.pest"]
pub(crate) struct CsvParser;

#[allow(dead_code)]
pub(crate) fn csv_example() {
    let unparsed_file = fs::read_to_string("numbers.csv").expect("Could not read file");

    let file = CsvParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut field_sum: f64 = 0.0;
    let mut record_count: u64 = 0;

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::record => {
                record_count += 1;

                for field in record.into_inner() {
                    field_sum += field.as_str().parse::<f64>().unwrap();
                }
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    println!("Sum of all fields: {}", field_sum);
    println!("Number of records: {}", record_count);
}
