use std::error::Error;
use std::hash::Hash;
use std::io::{BufRead, Read, Write};
use crate::benchmark::run_benchmark;
use crate::parsers::*;
use crate::generate_plot::*;
mod emitter;
mod benchmark;
mod parsers;
mod tools;
mod generate_plot;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "src/inputs/25mb.xml";
    let mut results = Vec::new();

    results.push(run_benchmark("whole_file", || {
        parse_xml_whole_file::parse(input_path, "src/outputs/whole_file.txt")?;
        Ok(())
    })?);

    results.push(run_benchmark("line_by_line", || {
        parse_xml_line_by_line::parse(input_path, "src/outputs/line_by_line.txt")?;
        Ok(())
    })?);

    results.push(run_benchmark("my_parser", || {
        my_parser::parse(input_path, "src/outputs/my_parser.txt")?;
        Ok(())
    })?);

    results.push(run_benchmark("xml-rs", || {
        xml_rs::parse(input_path, "src/outputs/xml-rs.txt")?;
        Ok(())
    })?);

    results.push(run_benchmark("quick-xml", || {
        quick_xml::parse(input_path, "src/outputs/quick-xml.txt")?;
        Ok(())
    })?);

    if results.is_empty() {
        return Err(Box::from("no results"));
    }

    generate_plot(&results)?;
    Ok(())
}


