(s) => {
    let except = "secondstate.io";
    let managers = [5794737289]
    let x = JSON.parse(s);
    
    if(x["message"]["entities"][0]["type"]=="url")
        if(!x["message"]["text"].includes(except))
            if(!managers.includes(x["message"]["from"]["id"])){
                return "warning "
                        +x["message"]["chat"]["id"]+" "
                        +x["message"]["from"]["id"];
            }
    
    return "";
}
