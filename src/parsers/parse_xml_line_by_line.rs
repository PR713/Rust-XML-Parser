use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use crate::emitter;
use crate::tools;

pub fn parse(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    start_parsing(&mut reader, &mut writer)?;
    Ok(())
}

pub fn start_parsing(
    reader: &mut BufReader<File>,
    writer: &mut BufWriter<File>,
) -> std::io::Result<()> {
    let mut buff_tag = String::new();
    let mut buff_text = String::new();
    let mut is_inside_tag = false;

    for line_result in reader.lines() {
        let line = line_result?;
        let mut chars = line.chars();

        while let Some(c) = chars.next() {
            if c == '<' {
                is_inside_tag = true;
                buff_text = buff_text.trim().to_string();
                if !buff_text.is_empty() {
                    emitter::text(writer, &buff_text)?;
                    buff_text.clear();
                }
            } else if c == '>' && is_inside_tag {
                is_inside_tag = false;
                tools::process_tag(&buff_tag, writer)?;
                buff_tag.clear();
            } else if is_inside_tag {
                buff_tag.push(c);
            } else {


                buff_text.push(c);

            }
        }

    }
    Ok(())
}
