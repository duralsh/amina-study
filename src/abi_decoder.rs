
use serde::{Deserialize, Serialize};
use std::fs;
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonItem {
    #[serde(default)]
    pub inputs: Vec<Input>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Input {
    #[serde(rename = "internalType")]
    pub internal_type: String,
    pub name: String,
    #[serde(rename = "type")]
    pub input_type: String,
}

pub fn read_abi(path: &str) -> Result<()> {
    let file_content = fs::read_to_string(path).expect("Unable to read the file");

    // Parse the JSON content
    let items: Vec<JsonItem> = serde_json::from_str(&file_content)?;

    for item in items.iter().filter(|&i| i.item_type == "function") {
        let input_types: Vec<String> = item.inputs.iter()
            .map(|input| input.input_type.clone())
            .collect();
    
        let input_types_str = if input_types.is_empty() {
            "()".to_string()
        } else {
            format!("({})", input_types.join(", "))
        };
    
        println!("function: {}{}", item.name, input_types_str);
    }

    Ok(())
}