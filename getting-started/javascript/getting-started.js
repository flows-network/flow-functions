(s) => {
    let payload = JSON.parse(s);
    if (payload["action"] === "created") {
        return payload["comment"]["body"].replace(/(?:\\[rn])+/g, "\n") + "\n"
            + payload["comment"]["html_url"];
    }
}
