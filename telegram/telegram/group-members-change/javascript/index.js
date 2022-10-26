(s) => {
    let payload = JSON.parse(s);
    if (payload["message"]["new_chat_members"])
        return JSON.stringify({
            chat_id: payload["message"]["chat"]["id"],
            text: "Welcome [" + payload["message"]["new_chat_members"][0]["first_name"]
                + "](tg://user?id=" + payload["message"]["new_chat_members"][0]["id"] + ")\\!",
            parse_mode: "MarkdownV2",
        });
    else if (payload["message"]["left_chat_member"])
        return JSON.stringify({
            chat_id: payload["message"]["chat"]["id"],
            text: "[" + payload["message"]["left_chat_member"]["first_name"]
                + "](tg://user?id=" + payload["message"]["left_chat_member"]["id"] + ") left\\!",
            parse_mode: "MarkdownV2",
        });
}
