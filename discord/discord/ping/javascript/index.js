(s) => {
    let message = JSON.parse(s);
    if(message["content"] === "/ping")
        return JSON.stringify({
            content: "PONG!",
            reply_to: message
        })
}
