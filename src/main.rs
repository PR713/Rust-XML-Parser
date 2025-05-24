mod utils;
mod parser;
mod emitter;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

fn main() -> std::io::Result<()> {
    let input_path = "input.txt";
    let output_path = "output.txt";

    // plik wejściowy
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    // plik wyjściowy
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    parser::start_parsing(&mut reader, &mut writer);

    Ok(())
}
