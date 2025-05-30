use std::borrow::Cow;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use quick_xml::events::Event;
use quick_xml::Reader;
use xml::EventReader;
use xml::reader::XmlEvent;
use sysinfo::{Pid, System};

mod parser;
mod emitter;

fn run_benchmark<F>(name: &str, f: F) -> Result<Duration, Box<dyn Error>>
where
    F: FnOnce() -> Result<(), Box<dyn Error>>,
{
    let pid = sysinfo::Pid::from(std::process::id() as usize);
    let max_memory = Arc::new(Mutex::new(0u64));
    let should_stop = Arc::new(Mutex::new(false));

    // Wątek monitorujący pamięć
    let memory_thread = {
        let max_memory = Arc::clone(&max_memory);
        let should_stop = Arc::clone(&should_stop);

        thread::spawn(move || {
            let mut system = System::new();
            let mut last_print = Instant::now();

            while !*should_stop.lock().unwrap() {
                system.refresh_process(pid);
                if let Some(process) = system.process(pid) {
                    let current_mem = process.memory() / 1024; // KB
                    let mut max_mem = max_memory.lock().unwrap();

                    if current_mem > *max_mem {
                        *max_mem = current_mem;
                    }

                    // Wypisuj co 100ms
                    if last_print.elapsed() > Duration::from_millis(100) {
                        // println!("[MEM] Current: {} KB, Peak: {} KB",
                        //          current_mem, *max_mem);
                        last_print = Instant::now();
                    }
                }
                thread::sleep(Duration::from_millis(50));
            }
        })
    };

    // Początkowy pomiar pamięci
    let initial_memory = {
        let mut system = System::new();
        system.refresh_process(pid);
        system.process(pid)
            .map(|p| p.memory() / 1024)
            .unwrap_or(0)
    };

    let start_time = Instant::now();
    let result = f();
    let duration = start_time.elapsed();

    // Zatrzymaj wątek monitorujący
    *should_stop.lock().unwrap() = true;
    memory_thread.join().unwrap();

    // Końcowe wyniki
    let final_max_memory = *max_memory.lock().unwrap();
    let memory_used_kb = final_max_memory.saturating_sub(initial_memory);

    // Wydruk wyników
    println!("\n╔═════════════════════════════════════════════════════════════════════╗");
    println!("║ {:^26} ║", name);
    println!("╠═════════════════════════════════════════════════════════════════════");
    println!("║ {:<12}: {:>10.2?}                                                   ", "Time", duration);
    println!("║ {:<12}: {:>10} KB   ", "RAM Memory (diff between the start and the end", memory_used_kb);
    println!("║                                                                     " );
    println!("║ {:<12}: {:>10} KB                                                   ", "Peak RAM", final_max_memory);
    println!("╚═════════════════════════════════════════════════════════════════════╝");

    result.map(|_| duration)
}


fn my_parser(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    parser::start_parsing(&mut reader, &mut writer);
    Ok(())
}

fn xml_rs(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}

fn quick_xml(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
                return Ok(())
            },
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

}

fn parse_xml_cow(input_path: &str, output_path: &str) -> std::io::Result<()> {
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


fn parse_xml_cow_optimized(input_path: &str, output_path: &str) -> std::io::Result<()> {
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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "treebank_e.xml";
    let output_path = "output.txt";

    run_benchmark("parse_xml_cow", || {
        parse_xml_cow(input_path, output_path)?;
        Ok(())
    });

    run_benchmark("parse_xml_cow_optimized", || {
        parse_xml_cow_optimized(input_path, output_path)?;
        Ok(())
    });

    run_benchmark("my_parser", || {
        my_parser(input_path, output_path)?;
        Ok(())
    });

    run_benchmark("xml-rs", || {
        xml_rs(input_path, output_path)?;
        Ok(())
    });

    run_benchmark("quick-xml", || {
        quick_xml(input_path, output_path)?;
        Ok(())
    });

    Ok(())
}
