use std::borrow::Cow;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

pub fn parse(input_path: &str, output_path: &str) -> std::io::Result<()> {
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
                // <?xml ... ?>
                if i + 1 < bytes.len() && bytes[i + 1] == b'?' {
                    i += 2;
                    let start = i;
                    while i + 1 < bytes.len() && !(bytes[i] == b'?' && bytes[i + 1] == b'>') {
                        i += 1;
                    }
                    let content = Cow::Borrowed(buffer[start..i].trim());
                    writeln!(writer, "Processing: <?{}?>", content)?;
                    i += 2; // skip '?>'
                }
                // </end>
                else if i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                    i += 2;
                    let start = i;
                    while i < bytes.len() && bytes[i] != b'>' {
                        i += 1;
                    }
                    let end = i;
                    let tag_name = Cow::Borrowed(buffer[start..end].trim());
                    writeln!(writer, "End: </{}>", tag_name)?;
                    i += 1; // skip '>'
                }
                // <start> lub <empty />
                else {
                    i += 1;
                    let start = i;
                    while i < bytes.len()
                        && !bytes[i].is_ascii_whitespace()
                        && bytes[i] != b'>'
                        && bytes[i] != b'/'
                    {
                        i += 1;
                    }
                    let end = i;
                    let tag_name = Cow::Borrowed(buffer[start..end].trim());

                    let mut attrs = Vec::new();
                    let mut is_empty = false;

                    // Atrybuty
                    while i < bytes.len() && bytes[i] != b'>' {
                        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                            i += 1;
                        }

                        if i < bytes.len() && bytes[i] == b'/' {
                            is_empty = true;
                            i += 1;
                            continue;
                        }

                        let attr_start = i;
                        while i < bytes.len() && bytes[i] != b'=' {
                            i += 1;
                        }
                        if i >= bytes.len() || bytes[i] != b'=' {
                            break;
                        }
                        let attr_key = Cow::Borrowed(buffer[attr_start..i].trim());
                        i += 1; // skip '='

                        if i < bytes.len() && bytes[i] == b'"' {
                            i += 1;
                            let val_start = i;
                            while i < bytes.len() && bytes[i] != b'"' {
                                i += 1;
                            }
                            let attr_val = Cow::Borrowed(buffer[val_start..i].trim());
                            attrs.push(format!("{}=\"{}\"", attr_key, attr_val));
                            i += 1; // skip closing '"'
                        }
                    }

                    let attr_str = if !attrs.is_empty() {
                        format!(" [{}]", attrs.join(", "))
                    } else {
                        String::new()
                    };

                    if is_empty {
                        writeln!(writer, "Empty: <{} />{}", tag_name, attr_str)?;
                    } else {
                        writeln!(writer, "Start: <{}>{}", tag_name, attr_str)?;
                    }

                    if i < bytes.len() && bytes[i] == b'>' {
                        i += 1; // skip '>'
                    }
                }
            }

            _ => {
                // Tekst między tagami
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
    Ok(())
}