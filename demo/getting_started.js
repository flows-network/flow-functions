(s) => {
    let x = JSON.parse(s);
    let body = "";
    let html_url = "";
    switch(x["action"]){
        case "created":
            body = x["comment"]["body"];
            html_url = x["comment"]["html_url"];
            break;
    }

    if (body) {
        return body.replace(/(?:\\[rn])+/g, "\n") + "\n" + html_url;
    } else {
        return "";
    }
}
