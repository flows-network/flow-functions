#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

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
