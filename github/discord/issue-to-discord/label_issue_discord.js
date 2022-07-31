(s) => {
    let x = JSON.parse(s);
    let issue = "";
    switch(x["action"]){
        case "opened":
            issue = "issue created";
            break;
        case "edited":
            issue = "issue edited";
            break;
        case "created":
            issue = "issue created";
            break;
        case "assigned":
            issue = "issue assigned";
            break;
    }

    return issue+"\n"+"label:\t"+x["issue"]["labels"][0]["name"]+"\n"+"issue body:\t"+x["issue"]["body"];
}