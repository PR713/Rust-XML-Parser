
mod parser;
mod emitter;

use std::borrow::Cow;
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
            XmlEvent::StartElement { name, attributes, .. } => {
                let attrs: Vec<String> = attributes
                    .iter()
                    .map(|a| format!("{}=\"{}\"", a.name.local_name, a.value))
                    .collect();
                let attr_str = if !attrs.is_empty() {
                    format!(" [{}]", attrs.join(", "))
                } else {
                    String::new()
                };
                writeln!(output, "Start: <{}>{}", name.local_name, attr_str)?;
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
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                let attrs: Vec<String> = e
                    .attributes()
                    .filter_map(|a| a.ok())
                    .map(|a| {
                        let key = String::from_utf8_lossy(a.key.as_ref()).to_string();
                        let value = a.unescape_value().unwrap_or_default().into_owned();
                        format!("{}=\"{}\"", key, value)
                    })
                    .collect();

                let attr_str = if !attrs.is_empty() {
                    format!(" [{}]", attrs.join(", "))
                } else {
                    String::new()
                };

                writeln!(writer, "Start: <{}>{}", tag_name, attr_str)?;
            }

            Ok(Event::Text(e)) => {
                writeln!(writer, "Text: {}", e.unescape()?.to_string())?;
            }
            Ok(Event::End(ref e)) => {
                writeln!(writer, "End: </{}>", String::from_utf8_lossy(e.name().as_ref()))?;
            }
            Ok(Event::Empty(ref e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                let attrs: Vec<String> = e
                    .attributes()
                    .filter_map(|a| a.ok())
                    .map(|a| {
                        let key = String::from_utf8_lossy(a.key.as_ref()).to_string();
                        let value = a.unescape_value().unwrap_or_default().into_owned();
                        format!("{}=\"{}\"", key, value)
                    })
                    .collect();

                let attr_str = if !attrs.is_empty() {
                    format!(" [{}]", attrs.join(", "))
                } else {
                    String::new()
                };

                writeln!(writer, "Empty: <{} />{}", tag_name, attr_str)?;
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

fn parse_xml_cow(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let start = Instant::now();
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let bytes = buffer.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'<' => {
                // sprawdź czy to tag zamykający
                if i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                    i += 2;
                    let start = i;
                    while i < bytes.len() && bytes[i] != b'>' {
                        i += 1;
                    }
                    let end = i;
                    let tag_name = Cow::Borrowed(&buffer[start..end]);
                    writeln!(writer, "End: </{}>", tag_name)?;
                    i += 1; // skip '>'
                } else {
                    i += 1;
                    let start = i;
                    while i < bytes.len() && !bytes[i].is_ascii_whitespace() && bytes[i] != b'>' {
                        i += 1;
                    }
                    let end = i;
                    let tag_name = Cow::Borrowed(&buffer[start..end]);

                    let mut attrs = Vec::new();
                    while i < bytes.len() && bytes[i] != b'>' {
                        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                            i += 1;
                        }

                        // attr name
                        let attr_start = i;
                        while i < bytes.len() && bytes[i] != b'=' {
                            i += 1;
                        }
                        if i >= bytes.len() {
                            break;
                        }
                        let attr_key = Cow::Borrowed(&buffer[attr_start..i]);
                        i += 1; // skip '='

                        if i < bytes.len() && bytes[i] == b'"' {
                            i += 1;
                            let val_start = i;
                            while i < bytes.len() && bytes[i] != b'"' {
                                i += 1;
                            }
                            let attr_val = Cow::Borrowed(&buffer[val_start..i]);
                            attrs.push(format!("{}=\"{}\"", attr_key, attr_val));
                            i += 1; // skip closing '"'
                        }
                    }

                    let attr_str = if !attrs.is_empty() {
                        format!(" [{}]", attrs.join(", "))
                    } else {
                        String::new()
                    };

                    writeln!(writer, "Start: <{}>{}", tag_name, attr_str)?;

                    i += 1; // skip '>'
                }
            }

            _ => {
                let start = i;
                while i < bytes.len() && bytes[i] != b'<' {
                    i += 1;
                }
                let end = i;
                let text = buffer[start..end].trim();
                if !text.is_empty() {
                    let content: Cow<str> = Cow::Borrowed(text);
                    writeln!(writer, "Text: {}", content)?;
                }
            }
        }
    }
    let duration = start.elapsed();
    println!("COW: Elapsed time: {:.2?}", duration);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let input_path = "treebank_e.xml";
    let output_path = "output.txt";

    my_parser(&input_path, &output_path)?;
    xml_rs(&input_path, &output_path)?;
    quick_xml(&input_path, &output_path)?;
    parse_xml_cow(&input_path, &output_path)?;
    Ok(())
}
