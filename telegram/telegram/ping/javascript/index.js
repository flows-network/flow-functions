(s) => {
    let payload = JSON.parse(s);
    if (payload["message"]["text"] === "/ping")
        return JSON.stringify({
            chat_id: payload["message"]["chat"]["id"],
            text: "__PONG\\!__",
            parse_mode: "MarkdownV2",
            reply_to_message_id: payload["message"]["message_id"]
        });
}
