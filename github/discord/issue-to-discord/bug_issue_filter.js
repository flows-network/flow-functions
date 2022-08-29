(s) => {
    let x = JSON.parse(s);
    if(x["issue"]["labels"][0]["name"] == "bug")
        return  "Label: "+x["issue"]["labels"][0]["name"]+"\n"+
                "Title: "+x["issue"]["title"]+"\n"+
                "Url: "+x["issue"]["html_url"];
    else
        return "";
}
