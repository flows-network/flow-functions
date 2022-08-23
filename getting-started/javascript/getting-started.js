(s) => {
    let x = JSON.parse(s);
    let html_url = "";
    let body = "";
    switch (x["action"]) {
        case "created":
            html_url = x["comment"]["html_url"];
            body = x["comment"]["body"]
            break;
    }
    return body.replace(/(?:\\[rn])+/g, "\n") + "\n" + html_url;
}