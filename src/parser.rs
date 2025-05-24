use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use crate::emitter;

// #[derive(Debug, PartialEq)]
// enum ParserState {
//     // OutsideTag,
//     // InsideTag,
//     Initial,
//     StartTag,
//     EndTag,
//     InsideText,
//     TagName,
//     AttributeName,
//     AttributeValue,
//     SelfClosingCheck,
//     Comment,
// }

pub fn start_parsing(reader : &mut BufReader<File>, writer : &mut BufWriter<File>){

    let mut buff_tag = String::new();
    let mut buff_text = String::new();

    let mut is_inside_tag = false;

    for byte in reader.bytes() {
        let c = byte.unwrap() as char;

        if c == '<' {
            is_inside_tag = true;
            if !buff_text.is_empty() {
                emitter::text();
                buff_text.clear();
            }
        }
        else if c == '>' {
            is_inside_tag = false;
            process_tag(&buff_tag, writer);
        }
        else if is_inside_tag{
            buff_tag.push(c);
        }
        else{
            buff_text.push(c);
        }
    }

}

fn process_tag(buff_tag : &String, writer : &mut BufWriter<File>){
    let mut parts = buff_tag.split_whitespace().collect::<Vec<&String>>();

    if buff_tag.starts_with('/') {
        emitter::end_tag();
    }
    else if buff_tag.ends_with('/') {
        let attr = get_attributes(&parts[1..parts.len()-1]);
        emitter::start_tag(writer, &parts[0], &attr);
        emitter::end_tag();
    }
    else{
        let attr = get_attributes(&parts[1..parts.len()]);
        emitter::start_tag(writer, &parts[0], &attr);
    }
}

fn get_attributes(parts : &[&String]) -> HashMap<String,String>{
    let mut map : HashMap<String,String> = HashMap::new();

    for part in parts{
        if let Some((k, v)) = part.split_once('='){
            map.insert(k.to_string(),v.to_string());
        }
    }

    map
}