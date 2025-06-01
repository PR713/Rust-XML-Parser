use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};


pub fn parse(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    for line_result in reader.lines() {
        let mut line = line_result?;
        let mut chars = line.char_indices().peekable();

        while let Some((i, c)) = chars.next() {
            if c == '<' {
                let mut tag_buffer = String::new();
                tag_buffer.push(c);

                // Zbieranie całego tagu
                while let Some((_, next_c)) = chars.next() {
                    tag_buffer.push(next_c);
                    if next_c == '>' {
                        break;
                    }
                }

                // Przetwarzanie tagu
                let tag_str = tag_buffer.trim();
                if tag_str.starts_with("</") {
                    // Zamykający tag
                    writeln!(writer, "End: {}", tag_str)?;
                } else {
                    // Otwierający lub pusty tag
                    let inside = &tag_str[1..tag_str.len() - 1]; // bez < i >
                    let mut parts = inside.split_whitespace();
                    if let Some(tag_name) = parts.next() {
                        let mut attributes = String::new();
                        for attr in parts {
                            attributes.push_str(attr);
                            attributes.push(' ');
                        }
                        attributes = attributes.trim().to_string();
                        if !attributes.is_empty() {
                            writeln!(writer, "Start: <{}> [{}]", tag_name, attributes)?;
                        } else {
                            writeln!(writer, "Start: <{}>", tag_name)?;
                        }
                    }
                }
            } else {
                // Tekst między tagami
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
                    let text_cow: Cow<str> = Cow::Borrowed(text);
                    writeln!(writer, "Text: {}", text_cow)?;
                }
            }
        }
    }
    Ok(())
}