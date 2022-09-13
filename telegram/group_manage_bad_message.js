(s) => {
    let x = JSON.parse(s);
    if(x["message"]["text"].includes("shit"))
        return  "ban "
                +x["message"]["chat"]["id"]+" "
                +x["message"]["from"]["id"]+" "
                +x["message"]["chat"]["title"]+" "
                +x["message"]["from"]["first_name"];
}
