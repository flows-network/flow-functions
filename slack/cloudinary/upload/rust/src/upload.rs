#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use serde_json::json;
use regex::Regex;

#[wasmedge_bindgen]
pub fn run_file_1(
    _: String, name: String, mimetype: String, file: Vec<u8>
) -> (String, String, String, Vec<u8>) {
    return (
        "".to_string(),     // (omit)
        name,               // File name without extension (required)
        mimetype,           // File MIME type (required)
        file,               // File data
    );
}

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let reg = Regex::new("<(\\S+)>").unwrap(); // Because slack will add angle brackets around the link
    return json!({
        "url": reg.captures(&s).unwrap().get(1).unwrap().as_str(),
        "file_type": "image"    // It must be image/video/raw, 
    }).to_string();
}
