use connector_dsi::github::inbound;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let workflow = payload.get_workflow_job()?;

    Ok(format!(
        "Workflow job {} {} triggered by {} in {}!\n{}Detil: {}",
        workflow.name,
        workflow.status,
        payload.sender.login,
        payload.get_repository()?.full_name,
        workflow
            .conclusion
            .as_ref()
            .map(|c| format!("Conclusion: {}\n", c))
            .unwrap_or_default(),
        workflow.html_url
    ))
}
