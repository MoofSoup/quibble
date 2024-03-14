use serde::{Serialize, Deserialize};

use serde_xml_rs::to_string;

#[derive(Serialize, Deserialize)]
struct Div {
    text: String,
    buttons: Vec<Button>,
    input: Input,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize)]
struct Button {
    label: String,
    // Add other fields as needed
}

#[derive(Serialize, Deserialize)]
struct Input {
    placeholder: String,
    // Add other fields as needed
}

fn serialize_div(div: &Div) -> Result<String, serde_xml_rs::Error> {
    let xml_string = to_string(div)?;
    Ok(xml_string)
}