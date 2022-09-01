#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use serde_json::json;
use regex::Regex;

#[wasmedge_bindgen]
pub fn run_file_1(
    exp: String, name: String, mimetype: String, pic: Vec<u8>
) -> (String, String, String, Vec<u8>) {
    return (
        "".to_string(),
        name,
        mimetype,
        pic,
    );
}

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let reg = Regex::new("<(\\S+)>").unwrap();
    return json!({
        "url": reg.captures(&s).unwrap().get(1).unwrap().as_str(),
        "file_type": "image"    // image | video | raw
    }).to_string();
}
