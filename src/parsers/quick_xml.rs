use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::emitter;

fn get_attributes(ref e: &BytesStart) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    let _ = e.attributes()
        .filter_map(|a| a.ok())
        .map(|a| {
            let key = String::from_utf8_lossy(a.key.as_ref()).to_string();
            let value = a.unescape_value().unwrap_or_default().into_owned();
            attrs.insert(key, value);
        });
    attrs
}

pub fn parse(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.trim_text(true);
    let mut writer = BufWriter::new(File::create(output_path)?);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                emitter::start_tag(&mut writer, &tag_name, &get_attributes(e))?;
            }
            Ok(Event::Text(e)) => {
                emitter::text(&mut writer, &e.unescape()?.to_string())?;
            }
            Ok(Event::End(ref e)) => {
                emitter::end_tag(&mut writer, &String::from_utf8_lossy(e.name().as_ref()))?;
            }
            Ok(Event::Empty(ref e)) => {
                emitter::start_tag(&mut writer, &String::from_utf8_lossy(e.name().as_ref()), &get_attributes(e))?;
                emitter::end_tag(&mut writer, &String::from_utf8_lossy(e.name().as_ref()))?;
            }
            Ok(Event::Eof) => {
                return Ok(())
            },
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }
}