(s) => {
    let x = JSON.parse(s);
    if(x["message"]["new_chat_member"])
        return  "new member \n"
                +"id: "+x["message"]["new_chat_member"]["id"]+"\n"
                +"first name: : "+x["message"]["new_chat_member"]["first_name"]+"\n"
                +"username: "+x["message"]["new_chat_member"]["username"]+"\n";
    if(x["message"]["left_chat_member"])
        return  "left member \n"
                +"id: "+x["message"]["left_chat_member"]["id"]+"\n"
                +"first name: : "+x["message"]["left_chat_member"]["first_name"]+"\n"
                +"username: "+x["message"]["left_chat_member"]["username"]+"\n";
    return ""
}
