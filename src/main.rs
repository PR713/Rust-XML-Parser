use std::error::Error;
use std::hash::Hash;
use std::io::{BufRead, Read, Write};
use crate::benchmark::run_benchmark;
use crate::parsers::*;
mod emitter;
mod benchmark;
mod parsers;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "treebank_e.xml";
    let output_path = "output.txt";

    run_benchmark("parse_xml_cow", || {
        parse_xml_cow::parse(input_path, output_path)?;
        Ok(())
    })?;

    run_benchmark("parse_xml_cow_optimized", || {
        parse_xml_cow_optimized::parse(input_path, output_path)?;
        Ok(())
    })?;

    run_benchmark("my_parser", || {
        my_parser::parse(input_path, output_path)?;
        Ok(())
    })?;

    run_benchmark("xml-rs", || {
        xml_rs::parse(input_path, output_path)?;
        Ok(())
    })?;

    run_benchmark("quick-xml", || {
        quick_xml::parse(input_path, output_path)?;
        Ok(())
    })?;

    Ok(())
}
