(s) => {
    let except = "secondstate.io";
    let managers = [5794737289]
    let payload = JSON.parse(s);

    if (payload["message"]["entities"][0]["type"] == "url" &&     // Is url
        !payload["message"]["text"].includes(except) &&           // Not contain expect
        !managers.includes(payload["message"]["from"]["id"])      // Not manager
    )
        return JSON.stringify({
            chat_id: payload["message"]["chat"]["id"],
            text: "Can't send strange url in this group\\!"
        })
}
