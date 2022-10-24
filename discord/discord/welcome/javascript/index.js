(s) => {
    let message = JSON.parse(s);
    if (message["type"] === 7)          // MemberJoin event
        return JSON.stringify({
            content: "Welcome <@" + message["author"]["id"] + ">!",
        })
}
