use std::error::Error;
use std::hash::Hash;
use std::io::{BufRead, Read, Write};
use crate::benchmark::run_benchmark;
use crate::parsers::*;
mod emitter;
mod benchmark;
mod parsers;
mod tools;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "src/inputs/25mb.xml";
    run_benchmark("parse_xml_whole_file", || {
        parse_xml_whole_file::parse(input_path, "src/outputs/parse_xml_whole_file.txt")?;
        Ok(())
    })?;

    run_benchmark("parse_xml_line_by_line", || {
        parse_xml_line_by_line::parse(input_path, "src/outputs/line_by_line.txt")?;
        Ok(())
    })?;

    run_benchmark("my_parser", || {
        my_parser::parse(input_path, "src/outputs/my_parser.txt")?;
        Ok(())
    })?;

    run_benchmark("xml-rs", || {
        xml_rs::parse(input_path, "src/outputs/xml-rs.txt")?;
        Ok(())
    })?;

    run_benchmark("quick-xml", || {
        quick_xml::parse(input_path, "src/outputs/quick-xml.txt")?;
        Ok(())
    })?;

    Ok(())
}


