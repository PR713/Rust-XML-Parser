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

        if c == '\n' || c == '\r' || c == '\t' {
            continue;
        }

        if c == '<' {
            is_inside_tag = true;
            if !buff_text.trim().is_empty() {
                emitter::text(writer, &buff_text);
                buff_text.clear();
            }
        }
        else if c == '>' {
            is_inside_tag = false;
            process_tag(&buff_tag, writer);
            buff_tag.clear();
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
    let mut parts = buff_tag.split_whitespace().collect::<Vec<&str>>();

    if buff_tag.starts_with('/') { //np </div>
        emitter::end_tag(writer, &buff_tag[1..]);
    }
    else if buff_tag.ends_with('/') { // <img src="" alt="" />
        let attr = get_attributes(&parts[1..parts.len()]);
        emitter::start_tag(writer, &parts[0], &attr);
        emitter::end_tag(writer, &parts[0]);
    }
    else{ // <div id="">
        let attr = get_attributes(&parts[1..parts.len()]);
        emitter::start_tag(writer, &parts[0], &attr);
    }
}

fn get_attributes(parts : &[&str]) -> HashMap<String,String>{
    let mut map : HashMap<String,String> = HashMap::new();

    for part in parts{
        if let Some((k, v)) = part.split_once('='){
            map.insert(k.to_string(),v.to_string());
        }
    }

    map
}