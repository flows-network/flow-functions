use wasmedge_bindgen_macro::*;
use flows_connector_dsi::github::inbound;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let repo = payload.get_repository()?;
    let r#ref: String = payload.get(&"ref")?;
    let ref_type: String = payload.get(&"ref_type")?;

    Ok(format!(
        "{}\n{}\n{}\n{}",
        format!("{} deleted", ref_type),
        r#ref,
        repo.description.as_ref().unwrap_or(&String::new()),
        repo.full_name
    ))
}
