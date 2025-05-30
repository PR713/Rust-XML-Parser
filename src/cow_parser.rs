use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Read, Write};
use crate::emitter;

pub fn start_parsing_cow<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> std::io::Result<()> {
    let mut buff_tag = String::new();
    let mut buff_text = String::new();
    let mut is_inside_tag = false;

    for byte in reader.bytes() {
        let c = byte? as char;

        if c == '\n' || c == '\r' || c == '\t' {
            continue;
        }

        if c == '<' {
            is_inside_tag = true;
            if !buff_text.is_empty() {
                let content: Cow<str> = Cow::Borrowed(&buff_text);
                emitter::text(writer, &content)?;
                buff_text.clear();
            }
        } else if c == '>' && is_inside_tag {
            is_inside_tag = false;
            let tag: Cow<str> = Cow::Borrowed(&buff_tag);
            process_tag_cow(tag, writer)?;
            buff_tag.clear();
        } else if is_inside_tag {
            buff_tag.push(c);
        } else {
            buff_text.push(c);
        }
    }

    Ok(())
}

fn process_tag_cow<W: Write>(tag: Cow<str>, writer: &mut W) -> std::io::Result<()> {
    let tag_str = tag.as_ref();
    let parts = tag_str.split_whitespace().collect::<Vec<&str>>();

    if tag_str.starts_with('/') {
        emitter::end_tag(writer, &tag_str[1..])
    } else if tag_str.ends_with('/') {
        let attr = get_attributes(&parts[1..]);
        emitter::start_tag(writer, parts[0], &attr)?;
        emitter::end_tag(writer, parts[0])
    } else {
        let attr = get_attributes(&parts[1..]);
        emitter::start_tag(writer, parts[0], &attr)
    }
}

fn get_attributes(parts: &[&str]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for part in parts {
        if let Some((k, v)) = part.split_once('=') {
            map.insert(k.to_string(), v.to_string());
        }
    }
    map
}
