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
    let before: String = payload.get(&"before")?;
    let after: String = payload.get(&"after")?;

    Ok(format!(
        "Repository: {}\nCommit: {} => {}\n\n{}",
        payload.get_repository()?.full_name,
        before,
        after,
        payload
            .get_commits()?
            .into_iter()
            .map(|commit| format!(
                "{}: {}",
                commit
                    .committer
                    .username
                    .as_ref()
                    .unwrap_or(&commit.committer.name),
                commit.message
            ))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}
