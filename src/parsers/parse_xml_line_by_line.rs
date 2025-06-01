use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use crate::emitter;

pub fn parse(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    for line_result in reader.lines() {
        let line = line_result?;
        let mut chars = line.char_indices().peekable();

        while let Some((i, c)) = chars.next() {
            if c == '<' {
                let mut tag_buffer = String::new();
                tag_buffer.push(c);

                // Collect the whole tag
                while let Some((_, next_c)) = chars.next() {
                    tag_buffer.push(next_c);
                    if next_c == '>' {
                        break;
                    }
                }

                // Process the tag
                let tag_str = tag_buffer.trim();
                if tag_str.starts_with("</") {
                    // End tag
                    let tag_name = tag_str[2..tag_str.len()-1].trim();
                    emitter::end_tag(&mut writer, tag_name)?;
                } else {
                    // Start tag
                    let inside = &tag_str[1..tag_str.len() - 1];
                    let mut parts = inside.split_whitespace();
                    if let Some(tag_name) = parts.next() {
                        let mut attributes = HashMap::new();

                        for attr in parts {
                            if let Some(eq_pos) = attr.find('=') {
                                let key = &attr[..eq_pos];
                                let value = &attr[eq_pos+1..]
                                    .trim_matches('"')
                                    .trim_matches('\'');
                                attributes.insert(key.to_string(), value.to_string());
                            }
                        }

                        emitter::start_tag(&mut writer, tag_name, &attributes)?;
                    }
                }
            } else {
                // Text between tags
                let text_start = i;
                let mut text_end = i + c.len_utf8();
                while let Some(&(j, next_c)) = chars.peek() {
                    if next_c == '<' {
                        break;
                    }
                    text_end = j + next_c.len_utf8();
                    chars.next(); // consume
                }

                let text = line[text_start..text_end].trim();
                if !text.is_empty() {
                    emitter::text(&mut writer, text)?;
                }
            }
        }
    }
    Ok(())
}