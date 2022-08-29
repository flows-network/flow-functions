(s) => {
    let x = JSON.parse(s);
    return  "Label:" +x["issue"]["labels"][0]["name"]+"\n"+
            "Title:" +x["issue"]["title"]+"\n"+
            "Url:" +x["issue"]["html_url"];
}
