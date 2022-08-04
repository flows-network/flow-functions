(s) => {
    let x = JSON.parse(s);
  
    if(x["event_type"] == "issue")
    {
        let issue = "issue " + x["object_attributes"]["action"] + "\n";
        let project = "project name: " + x["project"]["name"] + "\n";
        let info = "title: " + x["object_attributes"]["title"] + "\n" + "description: " + x["object_attributes"]["description"] + "\n";
        let homepage = "homepage: " + x["repository"]["homepage"] + "\n";
        let user = "user: " + x["user"]["username"];
        
        return issue + project + info + homepage + user;
    }
    else
    {
        return "";
    }
}