use wasmedge_bindgen_macro::*;
use connector_dsi::github::inbound;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let r#ref: String = payload.get(&"ref")?;
    let ref_type: String = payload.get(&"ref_type")?;
    let description: String = payload.get(&"description")?;

    Ok(format!(
        "{}\n{}\n{}\n{}",
        format!("{} created", ref_type),
        r#ref,
        description,
        payload.get_repository()?.full_name
    ))
}
