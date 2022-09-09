#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use serde_json::json;
use regex::Regex;
use url::Url;

#[wasmedge_bindgen]
pub fn run_file_1(
    path: String, name: String, mime_type: String, file: Vec<u8>
) -> (String, String, String, Vec<u8>) {
    let path = match path.is_empty() {
        true => path,
        false => format!("{}/", path)
    };

    return (
        format!("/{}{}", path, name),   // Path in the user's Dropbox to save the file. (required)
        name,                           // File name (optional)
        mime_type,                      // MIME type (optional)
        file,                           // File data (required)
    );
}

#[wasmedge_bindgen]
pub fn run(url: String) -> String {
    // Because slack will add angle brackets around the link
    let url = Regex::new("<(\\S+)>").unwrap()
        .captures(&url).unwrap().get(1).unwrap().as_str();
    
    let filename = url.parse::<Url>().unwrap().path_segments()
        .map(|it| it.last().unwrap().to_string())
            .unwrap_or("untitled".to_string());

    return json!({
        "path": format!("/{}", filename),
        "url": url
    }).to_string();
}
