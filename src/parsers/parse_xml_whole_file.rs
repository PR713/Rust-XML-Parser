use crate::emitter;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
//todo poprawka atrybutów
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
                // Pomijamy <?...?> (np. <?xml version="1.0"?>)
                if i + 1 < bytes.len() && bytes[i + 1] == b'?' {
                    i += 2;
                    while i + 1 < bytes.len() {
                        if bytes[i] == b'?' && bytes[i + 1] == b'>' {
                            i += 2;
                            break;
                        }
                        i += 1;
                    }
                    continue;
                }

                // </end>
                else if i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                    i += 2;
                    let start = i;
                    while i < bytes.len() && bytes[i] != b'>' {
                        i += 1;
                    }
                    let name = buffer[start..i].trim().to_string();
                    emitter::end_tag(&mut writer, &name)?;
                    i += 1;
                }

                // <start> lub <empty />
                else {
                    i += 1;
                    let start = i;
                    while i < bytes.len()
                        && !bytes[i].is_ascii_whitespace()
                        && bytes[i] != b'>' && bytes[i] != b'/'
                    {
                        i += 1;
                    }
                    let name = buffer[start..i].trim().to_string();

                    let mut attrs = HashMap::new();
                    let mut is_empty = false;

                    // Parsowanie atrybutów
                    while i < bytes.len() && bytes[i] != b'>' {
                        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                            i += 1;
                        }

                        if i < bytes.len() && bytes[i] == b'/' {
                            is_empty = true;
                            i += 1;
                            continue;
                        }

                        let key_start = i;
                        while i < bytes.len() && bytes[i] != b'=' {
                            i += 1;
                        }

                        if i >= bytes.len() || bytes[i] != b'=' {
                            break;
                        }

                        let key = buffer[key_start..i].trim().to_string();
                        i += 1; // skip '='

                        if i < bytes.len() && bytes[i] == b'"' {
                            i += 1;
                            let val_start = i;
                            while i < bytes.len() && bytes[i] != b'"' {
                                i += 1;
                            }
                            let value = buffer[val_start..i].to_string();
                            i += 1; // skip closing '"'
                            attrs.insert(key, value);
                        }
                    }

                    emitter::start_tag(&mut writer, &name, &attrs)?;
                    if is_empty {
                        emitter::end_tag(&mut writer, &name)?;
                    }

                    if i < bytes.len() && bytes[i] == b'>' {
                        i += 1;
                    }
                }
            }

            _ => {
                // Tekst między tagami
                let start = i;
                while i < bytes.len() && bytes[i] != b'<' {
                    i += 1;
                }
                let text = buffer[start..i].trim();
                if !text.is_empty() {
                    emitter::text(&mut writer, text)?;
                }
            }
        }
    }

    Ok(())
}
