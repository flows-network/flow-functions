use flows_connector_dsi::{github::inbound, sendgrid::outbound};
use rand::seq::SliceRandom;
use rand::thread_rng;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;

    let email = match payload.sender.email.as_ref() {
        Some(e) => e,
        None => return Ok(String::new()),
    };

    let sender = &payload.sender.login;
    let repo = &payload.get_repository()?.full_name;

    if payload.get_action()? == "created" {
        let body = pick_question();

        outbound(vec![email])
            .subject(
            "Thanks for your star! Let's get started with WebAssembly on the server",
            )
            .content(format!( r#"Hi {}!<br/>
Welcome to the {} community! Here are some resources to get you started. Please feel free to reach out to us on GitHub or Discord if you have questions or encounter any issues. Happy hacking!<br/>
<br/>
Please refer to our quick start guides to install WasmEdge and run your Wasm apps:<br/>
https://wasmedge.org/book/en/quick_start.html
<br/><br/>
The repo below contains a complete demo app in WasmEdge. It is a microservice with a web frontend and a MySQL database backend.<br/>
https://github.com/second-state/microservice-rust-mysql
<br/><br/>
WasmEdge has collaborated with Docker to create a seamless developer experience for building, sharing, and running applications with mixed Linux containers and Wasm sandboxes. You can read Docker's announcement below, and try the above microservice demo in Docker Desktop or Docker CLI!<br/>
https://www.docker.com/blog/docker-wasm-technical-preview/               
<br/><br/>
Finally, If you are in Detorit for KubeCon this week, please come by our kiosk in the CNCF projects area and say hi!
<br/><br/>
PS. Do you know ...<br/>
{}
<br/>
"#, sender, repo, body
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

pub fn pick_question() -> String {
    let q_1: &str = r#"Which foundation does WasmEdge belong to?
A CNCF
B LF Edge
"#;

    let q_2: &str = r#"Which AI frameworks has WasmEdge already supported? (multiple choice)
A TensorFlow
B Pytorch
C OpenVino
"#;

    let q_3: &str = r#"Tokio-based async apps can be compiled into Wasm and run in WasmEdge. Is this correct?
A Yes
B No
"#;

    let q_4: &str = r#"WasmEdge could run Nodejs applications. Is this correct?
A Yes
B No
C working in progress
"#;

    let q_5: &str = r#"WasmEdge supports both HTTP and HTTPS. Is this correct?
A Yes
B HTTP only
C HTTPS only
D None of them
"#;

    let q_6: &str = r#"Which relational database management system does WasmEdge support?(multiple choice)
A MySQL
B MariaDB
C TiDB
D Postgres
"#;

    let q_7: &str = r#"WasmEdge is OCI compatible and could be managed by K8s. Right?
A Yes
B No
"#;

    let q_8: &str = r#"Which platforms does WasmEdge support in the following list?(multiple choice)
A Android
B OpenWRT
C Sel4
D centos 7
"#;

    let q_9: &str = r#"What's the latest version of WasmEdge
A 0.10.0
B 0.11.0
C 0.11.1
D 0.11.2
"#;

    let q_10: &str = r#"When did WebAssembly become the fourth standard of Web
A 2016
B 2019
C 2020
D2021
"#;

    let arr = [q_1, q_2, q_3, q_4, q_5, q_6, q_7, q_8, q_9, q_10];
    arr.choose(&mut thread_rng()).unwrap().to_string()
}
