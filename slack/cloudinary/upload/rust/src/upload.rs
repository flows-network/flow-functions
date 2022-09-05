#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use regex::Regex;

#[wasmedge_bindgen]
pub fn run_file_1(
    _: String, name: String, mimetype: String, file: Vec<u8>
) -> (String, String, String, Vec<u8>) {
    return (
        "".to_string(),     // (omit)
        name,               // File name (optional)
        "".to_string(),     // (omit)
        file,               // File data (required)
    );
}

#[wasmedge_bindgen]
pub fn run_file_url(s: String) -> String {
    // Because slack will add angle brackets around the link
    let reg = Regex::new("<(\\S+)>").unwrap(); 

    // Return the file URL to Cloudinary
    return reg.captures(&s).unwrap().get(1).unwrap().as_str().to_string();
}
