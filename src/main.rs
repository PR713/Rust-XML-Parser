
mod parser;
mod emitter;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::time::Instant;
use quick_xml::events::Event;
use quick_xml::Reader;
use xml::EventReader;
use xml::reader::XmlEvent;

fn my_parser(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let start = Instant::now();

    // plik wejściowy
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    // plik wyjściowy
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    parser::start_parsing(&mut reader, &mut writer);
    let duration = start.elapsed();
    println!("Elapsed time: {:.2?}", duration);
    Ok(())
}

fn xml_rs(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let file = File::open(input_path)?;
    let file = BufReader::new(file);

    let parser =  EventReader::new(file);
    let mut output = BufWriter::new(File::create(output_path)?);

    for e in parser {
        match e? {
            XmlEvent::StartElement { name, .. } => {
                writeln!(output, "Start: <{}>", name.local_name)?;
            }
            XmlEvent::Characters(s) => {
                writeln!(output, "Text: {}", s)?;
            }
            XmlEvent::EndElement { name } => {
                writeln!(output, "End: </{}>", name.local_name)?;
            }
            _ => {}
        }
    }

    let duration = start.elapsed();
    println!("XML-rs: Elapsed time: {:.2?}", duration);

    Ok(())
}

fn quick_xml(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let file = File::open(input_path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.trim_text(true);

    let mut writer = BufWriter::new(File::create(output_path)?);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                writeln!(writer, "Start: <{}>", String::from_utf8_lossy(e.name().as_ref()))?;
            }
            Ok(Event::Text(e)) => {
                writeln!(writer, "Text: {}", e.unescape()?.to_string())?;
            }
            Ok(Event::End(ref e)) => {
                writeln!(writer, "End: </{}>", String::from_utf8_lossy(e.name().as_ref()))?;
            }
            Ok(Event::Eof) => {
                let duration = start.elapsed();
                println!("quick-xml: Elapsed time: {:.2?}", duration);
                return Ok(())
            },
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let input_path = "input.xml";
    let output_path = "output.txt";

    //my_parser(&input_path, &output_path)?;
    xml_rs(&input_path, &output_path)?;
    //quick_xml(&input_path, &output_path)?;
    Ok(())
}
