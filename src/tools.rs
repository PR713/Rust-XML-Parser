use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use crate::emitter;

pub fn process_tag(buff_tag: &String, writer: &mut BufWriter<File>) -> std::io::Result<()> {
    let mut parts = buff_tag.split_whitespace().collect::<Vec<&str>>();

    if buff_tag.starts_with('/') {
        //np </div>
        emitter::end_tag(writer, &buff_tag[1..])?;
    } else if buff_tag.ends_with('/') {
        // <img src="" alt="" />
        let attr = get_attributes(&parts[1..parts.len()]);
        emitter::start_tag(writer, &parts[0], &attr)?;
        emitter::end_tag(writer, &parts[0])?;
    } else {
        // <div id="">
        let attr = get_attributes(&parts[1..parts.len()]);
        emitter::start_tag(writer, &parts[0], &attr)?;
    }
    Ok(())
}

pub fn get_attributes(parts: &[&str]) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    for part in parts {
        if let Some((k, v)) = part.split_once('=') {
            map.insert(k.to_string(), v.to_string());
        }
    }
    map
}