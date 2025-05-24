use std::io::Write;

pub fn start_tag<W: Write>(
    writer: &mut W,
    name: &str,
    attrs: &std::collections::HashMap<String, String>,
) -> std::io::Result<()> {
    let attr_str = attrs
        .iter()
        .map(|(k, v)| format!(r#""{}":"{}""#, k, v))
        .collect::<Vec<_>>()
        .join(",");

    writeln!(
        writer,
        r#"{{"type":"start_element","name":"{}","attributes":{{{}}}}}"#,
        name, attr_str
    )
}

pub fn end_tag(){}


pub fn text(){

}