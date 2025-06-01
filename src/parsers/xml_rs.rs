use std::io::Write;

use crate::emitter;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use xml::EventReader;
use xml::reader::XmlEvent;

pub fn parse(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let parser = EventReader::new(BufReader::new(file));
    let mut writer = BufWriter::new(File::create(output_path)?);

    for e in parser {
        match e? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                let mut attrs = HashMap::new();
                for a in attributes {
                    attrs.insert(a.name.local_name, a.value);
                }
                emitter::start_tag(&mut writer, &name.to_string(), &attrs)?;
            }
            XmlEvent::Characters(s) => {
                emitter::text(&mut writer, &s)?;
            }
            XmlEvent::EndElement { name } => {
                emitter::end_tag(&mut writer, &name.to_string())?;
            }
            _ => {}
        }
    }

    Ok(())
}
