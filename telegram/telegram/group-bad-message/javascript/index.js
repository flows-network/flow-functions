(s) => {
    let payload = JSON.parse(s);
    if(payload["message"]["text"].includes("shit"))
        return JSON.stringify({
            chat_id: payload["message"]["chat"]["id"],
            user_id: payload["message"]["from"]["id"]
        });
}
