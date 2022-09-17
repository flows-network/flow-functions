use connector_dsi::{github::inbound, sendgrid::outbound};
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let email = match payload
        .starred_at
        .as_ref()
        .and(payload.sender.email.as_ref())
    {
        Some(e) => e,
        None => return Ok(String::new()),
    };

    let sender = &payload.sender.login;
    let repo = &payload.get_repository()?.full_name;

    if payload.get_action()? == "created" {
        outbound(vec![email])
            .subject(" 😉 Thank you for your star!")
            .content(format!(
                r#"
Howdy {},<br/>
                
Welcome to the {} community! Thank you so much for starring our repository. We are very excited to have you here.<br/>

Look forward to your contributions,<br/>
Maintainers at repository {}"#,
                sender, repo, repo
            ))
            .build()
    } else {
        outbound(vec![email])
            .subject(" 😿 Sorry to lose you")
            .content(format!(
                r#"
Hi {},<br/>

Sorry to see you go! We value your feedback and suggestions. Please do let us know how we might improve the repository {} (just reply to this email). We wish to see your around in the community!<br/>

Best Regards,<br/>
Maintainers at repository {}"#,
                sender, repo, repo
            ))
            .build()
    }
}
