(s) => {
    let x = JSON.parse(s);
    if(x["message"]["new_chat_member"])
        return  "welcome "
                +x["message"]["chat"]["id"]+" "
                +x["message"]["new_chat_member"]["id"]+" "
                +x["message"]["new_chat_member"]["username"];
    return ""
}
