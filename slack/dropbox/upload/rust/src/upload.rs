#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run_file_1(
    path: String, name: String, mime_type: String, file: Vec<u8>
) -> (String, String, String, Vec<u8>) {
    return (
        format!("/{}/{}", path, name),  // Path in the user's Dropbox to save the file. (required)
        name,                           // File name (optional)
        mime_type,                      // MIME type (optional)
        file,                           // File data (required)
    );
}
